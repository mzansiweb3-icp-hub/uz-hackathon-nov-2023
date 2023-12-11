# Micro DeFi

### Defi project where users can exchange their tokens with ease

## Prerequisites

- Follow these instructions from the Azle Book to [install all the required tools and set up your environment](https://demergent-labs.github.io/azle/installation.html). If you have already installed the required tools, you can skip this step.

After you have installed the required tools, you can move on to the next step.

## Clone the repository

```bash

Clone this repository and navigate to the `micro_defi` directory:
```

```bash
git clone https://github.com/sameicp/uz-hackathon-nov-2023/tree/micro-defi
cd micro_defi
```

## Install dependencies

```bash
npm install
```

## Start the local replica

```bash
dfx start --clean --background
```

Deploy the canister locally:

```bash
dfx deploy
```

## Interact with the canister

You can now interact with the canister using the the candid interface URL provided in the output above.

You can run the following command to interact with frontend

```bash
npm run start
```

# Link to

- [YouTube video](https://youtu.be/69FpEYvmBCs)
- [Frontend Canister](https://6dzvv-miaaa-aaaak-qcwgq-cai.icp0.io/)
- [Backend Canister](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=6k26j-2aaaa-aaaak-qcwha-cai)

# Improvements to be done on the project

- Since the projects make use of some dummy values to represent ckEth and the ICP tokens, the major improvements is going to implement the ICP ledger for ICP tokens and use Sepolia testnet Eth to mint ckETH.
- ReImplement the project with Motoko which is a more stable programming language and has a bigger community for support ans Motoko is considered more Stable than Typescript
