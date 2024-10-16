# Defi 101

This is a simple example of a constant product automated market maker (AMM) on the Solana blockchain. The smart contract is deployed to the devnet and can be used to swap between two tokens.

The aim is to continue building it in our community and eventually move it to the mainnet.

## Dependencies

- [anchor](https://www.anchor-lang.com/docs/installation)
- [deno](https://docs.deno.com/runtime/getting_started/installation/)

## Technologies

Smart contracts on Solana are written in Rust, then compiled and deployed using Anchor. Anchor is a framework that makes it easier to develop on Solana.

Deno is a TypeScript runtime that is used to run tests and interact with the smart contract. It can be easily run in the browser or in a Node.js environment.

## Usage

### Build and test
```bash
anchor build
anchor test
```

### Deploy

```bash
anchor deploy --provider.cluster devnet
```

### Interact

```bash
deno run --allow-net scripts/init.ts
deno run --allow-net scripts/deposit.ts
deno run --allow-net scripts/swap.ts
```

## Glossary 

- **Token**: A token is a digital asset on Solana that can be used to pay for goods and services.
- **AMM**: Automated Market Maker - A type of decentralized exchange (DEX) that allows users to exchange tokens.
- **LP**: Liquidity Provider - A user who provides liquidity to an AMM pool.
- **LP token**: A token that represents a share of the pool. It is used to receive fees and rewards.
- **Vault**: A vault is a smart contract that holds the assets of a user.
- **Swap**: A swap is a transaction that allows a user to exchange one token for another.
- **Deposit**: A deposit is a transaction that allows a user to add liquidity to an AMM pool.
- **Withdraw**: A withdraw is a transaction that allows a user to remove liquidity from an AMM pool.
- **Smart Contract**: A smart contract is a program that runs on the Solana blockchain. It is a self-executing contract with the terms of the agreement directly written into code.


## TODO

This is a basic proof of concept. There are many things to improve:

- implement the constant product formula
- allow for multiple token pairs in a single contract
- audit security of the contract (left some vulnerabilities on purpose)
- add a fee to the swap
- add a minimalistic frontend
- extent the test suite to multiple Liquidity Providers (LPs)

## License

MIT