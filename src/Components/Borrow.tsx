import { Console } from 'console';
import React, { useState, useEffect } from 'react';
import { useNavigate } from "react-router-dom";

function Borrow({ backendActor, principal, isAuthenticated }) {
  const [deposit, setDeposit] = useState(0);
  const [amtToBorrow, setAmount] = useState(0);
  const [isSending, setIsSending] = useState(false);
  const [amtBorrowed, setAmtBorrowed] = useState(null);
  const [hasDebt, setHasDebt] = useState(false);
  const [deposited, setDepositCollateral] = useState();
  const navigate = useNavigate();

  async function fetchData() {
    try {
      // Fetch the deposited amount from the backendActor
      const deposited = await backendActor.getCollateralIcpToken(principal);
      const debt = await backendActor.getHasDebt(principal);
      setHasDebt(debt[0]);
      setDepositCollateral(deposited[0]);
    } catch (error) {
      console.log(error);
    }
  }

  useEffect(() => {
    fetchData();
  }, []);

  const makeDeposit = async (e) => {
    e.preventDefault();
    try {
      setIsSending(true);
      await backendActor.depositCollateral(principal, deposit);
      setDeposit(0);
      setIsSending(false);
      fetchData()
    } catch (error) {
      console.log(error);
      setIsSending(false);
    }
  };

  const borrowTokens = async (e) => {
    e.preventDefault();
    try {
      setIsSending(true);
      await backendActor.borrowTokens(principal, amtToBorrow);
      setAmtBorrowed(amtToBorrow);
      navigate('/dashboard');
    } catch (error) {
      console.log(error);
      navigate('/error-page');
    } finally{
        setIsSending(false);
    }
  };
  if (!isAuthenticated) {
    // If user is not authenticated, redirect to home page or handle accordingly
    navigate('/');
  }


  return (
    <div className="flex justify-center items-center h-screen">
      <div className="mx-auto max-w-lg p-6 bg-gray-600 rounded-md shadow-md">
        <h2 className="text-2xl font-bold mb-4">{(deposited || -1) <= 0 ? "Deposit ICP Tokens" : "Borrow ckETH Tokens"}</h2>
        {(deposited || -1) <= 0? (
          <form onSubmit={makeDeposit} className="mb-4">
            <label className="block mb-2">Deposit Amount:</label>
            <input
              type="number"
              value={deposit}
              onChange={(e) => setDeposit(Number(e.target.value))}
              className="text-black border border-gray-300 rounded-md px-2 py-1 mb-2"
            />
            <button
              type="submit"
              className="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded block mx-auto"
            >
              {!isSending? "Deposit" : "Depositing..."}
            </button>
          </form>
        ) : hasDebt ? (
            <div>You already have debt to cover.</div>
        ) : (
            <form onSubmit={borrowTokens}>
                <label className="block mb-2">Amount to Borrow:</label>
                <input
                type="number"
                value={amtToBorrow}
                onChange={(e) => setAmount(Number(e.target.value))}
                className="text-black border border-gray-300 rounded-md px-2 py-1 mb-2"
                />
                <button
                type="submit"
                className="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded block mx-auto"
                >
                {!isSending? "Borrow" : "Borrowing..."}
                </button>
            </form>
        )}

        {amtBorrowed && (
          <div className="mt-4">
            <p>Borrowed Amount: {amtBorrowed}</p>
          </div>
        )}
      </div>
    </div>
  );
}

export default Borrow;
