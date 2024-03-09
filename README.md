# Wagerr Smart Contract

## Introduction

This repository contains the source code for the Wagerr smart contract written in Ink!, a Rust-based smart contract language for the Polkadot ecosystem. 

## Build and Deploy

> **Pre-requisites:**
>
> - Install Rust via the [Substrate Docs](https://docs.substrate.io/install/) (skip the "Compile a Substrate node" section)
> - Install [`cargo contract`](https://github.com/paritytech/cargo-contract)
> - Install [`substrate-contracts-node`](https://github.com/paritytech/substrate-contracts-node)

```bash
# Install dependencies
pnpm install

# Build contracts and move artifacts to `contracts/deployments/{contract}/` folders
pnpm run build

# Start local node with persistence (contracts stay deployed after restart)
# NOTE: When using Brave, shields have to be taken down for the UIs
pnpm run node

## IMPORTANT: Open a separate terminal window and keep the node running

# Deploy the contracts on the local node
pnpm run deploy
```

Alternatively, you can also deploy contracts manually using [Contracts UI](https://contracts-ui.substrate.io/) (`pnpm contracts-ui`) in the browser.


## Interacting with the Smart Contract

Once deployed, users can interact with the Wagerr Smart Contract in various ways:

1. `createWager(name: String, terms: String)`: Create a new wager
2. `getWager(id: String)`: Retrieve a wager with id.
3. `getPendingWagers()`: Get all pending wagers for an active account.
4. `getActiveWagers()`: Get all active wagers for an active account.
5. `joinWager(id: String)`: Join a pending wager.
6. `claimWin(id: String)`: Claim win for an active wager.
7. `AcceptRejectClaim(id: String, action: ClaimAction)`: Accept or Reject other party's claim.

## Testing

## Contributing

Contributions to the Wagerr are welcome! Feel free to fork this repository, make your changes, and submit a pull request. Make sure to follow the contribution guidelines outlined in the repository.
