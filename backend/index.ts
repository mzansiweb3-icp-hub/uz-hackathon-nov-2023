import { Canister, StableBTreeMap, query, text, update, Opt, Principal, nat, int, ic, Some, None, Void, bool, float64, Duration, TimerId } from 'azle';
import {managementCanister, HttpResponse, HttpTransformArgs} from 'azle/canisters/management';

// This is a global variable that is stored on the heap
const LIQUIDATION_THRESHOLD = 50n; // 200% overcollateralized
const LIQUIDATION_PRECISION = 100n;
const MIN_HEALTH_FACTOR: nat = BigInt(1e18);
const PRECISION = 1e18;
const DUE_DATE_MONTH: nat = 1000n * 60n * 60n * 24n * 30n

const INTEREST_RATE = 5n;
let ethereumToUsd = 2_000n;
let icpToUsd = 6n;

let ckEthPool = 0n;
let debtCollectionckETH = 0n;
let debtColletionICP = 0n;


////////////////////////
// StableBTreeMpas   ///
////////////////////////

let s_collateralDeposited = StableBTreeMap<Principal, nat>(Principal, nat, 7);
let s_lender = StableBTreeMap<Principal, nat>(Principal, nat, 8);
let s_debt = StableBTreeMap<Principal, nat>(Principal, nat, 9);
let hasDebt = StableBTreeMap<Principal, bool>(Principal, bool, 10);
let debtTimer = StableBTreeMap<Principal, TimerId>(Principal, TimerId, 11);

/**
 * Project: Micro-Defi
 * Developer: @sameicp
 * Special Thanks: @iamenochchirima and @renegadec for the help offered when it comes to understading azle
 * About the project: The projects is a DeFi market place where the users ICP tokens can stake their tokens
 * and buy ckEth on the IC and they later have to repay the ckEth tokens before they get liquidated
 * 
 * N.B: Faced some dificult when trying to implement the project using ckEth from sepolia and ended up using
 * dummy values for the tokens and the token prices. This projects shows the logic of how the tokens
 * will be processed and I am planning to continue with the project so that I can use the Eth from
 * testnet. Used dummy values for the price also because had some dificulty to change the blob response
 * from Http response into the real price of the token. So I will also have to find a way to use the 
 * price values in real time
 * 
 * the values of the tokens have 18 digits like 1 ETH = 1e18 wei (in Ethereum)
 * this is done because since the azle supports BigInt is struggles with decimal places so
 * we change the values to 18 degits for precision purpose
 * 
 * Special thanks to: ICP Hub SA and Uz Computer Science Department for the Opportunity
 * 
 */
export default Canister({
    
    /**
     * @param userId: userId assigned by Internet Identity
     * @param deposit: The amount of ICP tokens the user want to deposit
     * @returns amount of collateral deposited
     * This functions allows ICP holders to deposit their tokens so that
     * they can borrow ckEth tokens to do their trade
     */
    depositCollateral: update([text, float64], nat, (userId, deposit)=>{
        // check if collateral is more than zero
        if( deposit <= 0){
            throw new Error('can not deposit 0 ICP as collateral')
        }

        if( userId === ''){
            throw new Error('no user identity')
        }

        const depositWithPrecision: nat = BigInt(deposit) * BigInt(PRECISION);
        const userPrincipal: Principal = Principal.fromText(userId);
        s_collateralDeposited.insert(userPrincipal, depositWithPrecision);
        hasDebt.insert(userPrincipal, false);
        return depositWithPrecision;
    }),

    /**
     * @param userId The userId of a person who want to borrow ckETH
     * @param amountToBorrow The number of ckEth tokens they want to borrow
     * @returns the amount the user has borrowed
     * Functions allows the users who have deposited collateral to borrow
     * ckEth
     */
    borrowTokens: update([text, float64], nat, async (userId, amountToBorrow)=>{
        if( userId === ''){
            throw new Error('no user id');
        }

        if(amountToBorrow <= 0n){
            throw new Error('can not borrow zero amount')
        }

        if(amountToBorrow > ckEthPool){
            throw new Error('user can not borrow because of lack of the token');
        }

        const userPrincipal: Principal = Principal.fromText(userId);
        if(hasDebt.get(userPrincipal).Some){
          throw new Error('User already on debt');
        }

        const amtWithPrecision = BigInt(amountToBorrow ) * BigInt(PRECISION);
        revertIfHealthFactorIsBroken(userPrincipal, amtWithPrecision);
        s_debt.insert(userPrincipal, amtWithPrecision);
        hasDebt.insert(userPrincipal, true)
        ckEthPool -= amtWithPrecision;
        const deuDateId = setDueDate(userPrincipal, DUE_DATE_MONTH);
        debtTimer.insert(userPrincipal, deuDateId);
        return amtWithPrecision;
        
    }),

    /**
     * @param userId The Identity of the User
     * @param amountToLend The amount of ckEth tokens that the User want to invest
     * @returns ckEth tokens Staked
     * On the Platform, There will be borrowers and lenders.
     * Lenders will use the functions to lend their tokens for profit
     */
    lendckEthToken: update([text, float64], nat,(userId, amountToLend)=>{
      if(userId ===''){
        throw new Error('no userId');
      }

      if(amountToLend <= 0){
        throw new Error('no eth to lend')
      }
      
      const userPrincipal: Principal = Principal.fromText(userId);
      const amtToLendWithPrecison: nat = BigInt(amountToLend) * BigInt(PRECISION)
      s_lender.insert(userPrincipal, amtToLendWithPrecison);
      ckEthPool += amtToLendWithPrecison;
      return amtToLendWithPrecison;
    }),

    /**
     * @param liquidatorId Id for the person who want to liquidate another participant
     * @param victimId Id of the person who is going to be LIQUIDATED
     * 
     * This function allows participant to liquidate others so that thave failed to pay theiry debt
     */
    liquidation: update([text, text, float64], Void, (liquidatorId, victimId)=>{
      if( liquidatorId==='' || victimId===''){
        throw new Error('no liquidator of victim')
      }

      const liquidatorIcpOpt = s_collateralDeposited.get(liquidatorId);
      const victimIcpOpt = s_collateralDeposited.get(victimId);
      const victimDebtOpt = s_debt.get(liquidatorId);

      if('None' in victimDebtOpt || 'None' in victimIcpOpt){
        throw new Error('cannot liquidate a user without debt')
      }

      if('None' in liquidatorIcpOpt){
        throw new Error('can not liquidate a user if you do not have ICP')
      }

      const liquidatorIcp: nat = liquidatorIcpOpt.Some;
      let liquidatorDebt: nat;
      if('None' in s_debt.get(liquidatorId)){
        liquidatorDebt = 0n;
      } else{
        liquidatorDebt = s_debt.get(liquidatorId).Some;
      }

      // check if victim has debt
      const victimDebt: nat = victimDebtOpt.Some;
      const victimIcp: nat = victimIcpOpt.Some;

      // check if the victim healthfactor is broken
      const isVictimsHFBroken: bool = isHealthFactorBroken(victimIcp, victimDebt);

      if(!isVictimsHFBroken){
        throw new Error('can not liquidate user with good health factor');
      }

      // check liquidator hf
      const isLiquidatorHfBroken: bool = isHealthFactorBroken(liquidatorIcp, liquidatorDebt);

      if(isLiquidatorHfBroken){
        throw new Error('can not liquidate with bad health factor');
      }

      s_debt.insert(victimId, 0n);
      const updateLequidatorICP: nat = liquidatorIcp - victimDebt;
      s_collateralDeposited.insert(liquidatorId, updateLequidatorICP)
      
    }),

    /**
     * @param userId Id of the person who wants to pay the debt
     * @param amount the figure which the user has to reoay back
     * 
     * the function is used by person who has debt and needs to repay the debt with in 30 days
     */
    repayDebt: update([text, float64], Void, (userId, amount)=>{
      if( userId === ''){
        throw new Error('no user id');
      }

      if(amount <= 0){
          throw new Error('can not repay with zero amount')
      }

      const amtWithPrecision = BigInt(amount ) * BigInt(PRECISION);
      const userPrincipal: Principal = Principal.fromText(userId);
      const amountToPay = calcDebtWithInterest(userPrincipal);

      if(amtWithPrecision < amountToPay){
        throw new Error(`You are required to pay ${amountToPay / BigInt(PRECISION)}`);
      }

      const timerIdOpt = debtTimer.get(userPrincipal);

      if('None' in timerIdOpt){
          throw new Error(`Failed to fetch timerId for the user with ID: ${userPrincipal}`)
      }

      const timerId: TimerId = timerIdOpt.Some;
      // clear the timer if the user has payed the debt
      ic.clearTimer(timerId);
      debtCollectionckETH += amountToPay;
      s_debt.insert(userPrincipal, 0);
      hasDebt.insert(userPrincipal, false);

    }),

    /**
     * @param userId The id of thw user whom we want to check health factor foe
     * checks the Helath factor of the debtor if it is good or bad.
     */
    getHealthFactor: update([text], text, (userId)=>{
      const userPrincipal: Principal = Principal.fromText(userId);
      const userDebt: nat = s_debt.get(userPrincipal).Some;
      const userCollateral: nat = s_collateralDeposited.get(userPrincipal).Some
      const healthFactor = isHealthFactorBroken(userCollateral, userDebt);

      if(healthFactor){
        return 'bad';
      } else {
        return 'good'
      }

    }),

    // the function queries the if the user has debt and return true or false
    // depending on the debt status of the user
    getHasDebt: query([text], Opt(bool), (userId)=>{
      const userPrincipal: Principal = Principal.fromText(userId);
      return hasDebt.get(userPrincipal);
    }),

    // fi the user has debt then this function retrieves the debt that the user has
    getDebtInformation: query([text],Opt(nat), (userId)=>{
      const userPrincipal: Principal = Principal.fromText(userId);
      return s_debt.get(userPrincipal);
    }),

    // this function retrives the collateral staked by the user so that they can borrow ckEth
    getCollateralIcpToken: query([text], Opt(nat),(userId)=>{
      const userPrincipal: Principal = Principal.fromText(userId);
      return s_collateralDeposited.get(userPrincipal);
    }),

    // returns the total amount of ckEth available for users to borrow
    getckEthPool: query([], nat, ()=>{
      return ckEthPool;
    }),

    // this function retrieves the amount that the user has landed on the 
    // platform for profit
    getAmountLanded: query([text], Opt(nat), (userId)=>{
      const userPrincipal: Principal = Principal.fromText(userId);
      return s_lender.get(userPrincipal);
    })
});

const revertIfHealthFactorIsBroken = (userId: Principal, amt: nat)=>{
    const depositOpt = s_collateralDeposited.get(userId);

    if( 'None' in depositOpt){
        throw new Error('Can not find the deposits with the id');
    }

    const deposit: nat = depositOpt.Some;
    const healthFactor: nat = calculateHealthFactor(deposit, amt);

    if( healthFactor < MIN_HEALTH_FACTOR){
        throw new Error('Can not borrow because you have low collateral')
    }
}

// the function takes number of token and price of a single token
// the functions multiply the two numbers to get the usd value of the tokens
const convertTokensToUsd = (token: nat, usdPrice: nat)=>{
    return BigInt(token * usdPrice);
}

// calculates if the borrower is elligible to borrow the tokens
// the function compares the deposit and borrowedAmt and the deposit
// must always be greater than the value of token borrowed
const calculateHealthFactor = (deposit: nat, borrow: nat): nat=>{
    const depositInUsd: nat = convertTokensToUsd(deposit, icpToUsd);
    const fundsToBorrowInUsd: nat = convertTokensToUsd(borrow, ethereumToUsd);
    const depositAdjuctedForThreshold: nat = (depositInUsd * LIQUIDATION_THRESHOLD) / LIQUIDATION_PRECISION;
    const healthFactor: nat = (depositAdjuctedForThreshold * BigInt(PRECISION)) / fundsToBorrowInUsd;
    return healthFactor;
}

// when the user id repaying the debt they are supposed to with an interest added
// the function takes in userId as parameter
const calcDebtWithInterest = (userId: Principal): nat => {
    const userDebtOpt = s_debt.get(userId);
    if('None' in userDebtOpt){
      throw new Error('No debt found for the this user');
    }
    const userDebt = userDebtOpt.Some;
    const interest = (userDebt * INTEREST_RATE) / LIQUIDATION_PRECISION;
    const debtToPay = userDebt + interest;
    return debtToPay;
}

// the function check to see if the Health factor is broken
// functions returns a boolean
const isHealthFactorBroken = (icp: nat, debt: nat): boolean=>{
  const healthFactor = calculateHealthFactor(icp, debt);
  if(!debt){
    return false;
  }
  if(healthFactor > MIN_HEALTH_FACTOR){
    return false;
  }
  return true;
}

/// this function set the due date using the ic.setTimer function
/// the function takes in the id and the duration period to pay the debt
const setDueDate = (userId: Principal, period: Duration): TimerId =>{
  return ic.setTimer(period, ()=>{
      debtColletionICP += s_collateralDeposited.get(userId).Some;
      hasDebt.insert(userId, false);
      s_collateralDeposited.insert(userId, 0n);
  });
}
