# Stellar Interchain Token Service (ITS) Example

## Overview

This repository contains a complete example of how to use Axelar's Interchain Token Service with Stellar. It demonstrates how to create tokens that can exist across multiple blockchains and transfer them seamlessly between chains.

The example implements a wrapper contract for the Interchain Token Service that allows you to:
- Deploy new tokens on Stellar
- Deploy those tokens to other blockchains (like Avalanche)
- Transfer tokens between blockchains
- Register existing tokens for cross-chain use

## Features

- **Token Creation**: Deploy new tokens on Stellar with customizable parameters
- **Cross-Chain Deployment**: Make your tokens available on other blockchains
- **Token Transfers**: Send tokens between Stellar and other blockchains
- **Existing Token Registration**: Register existing Stellar tokens for cross-chain use

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) with wasm32 target
- [Stellar CLI](https://github.com/stellar/stellar-cli)
- A Stellar testnet account with funds
- [Metamask](https://metamask.io/) or [Rabby](https://rabby.io/) wallet for EVM chains
- Testnet tokens for destination chains (e.g., Avalanche Fuji testnet AVAX)

## Project Structure

```
axelar-its-app/
├── src
│   ├── contract.rs       # Main contract implementation
│   ├── error.rs          # Error definitions
│   ├── lib.rs            # Library entry point
│   └── storage_types.rs  # Storage definitions
├── Cargo.toml            # Dependencies and project configuration
├── README.md             # This file
```

## Setup and Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/axelarnetwork/stellar-its-example.git
   cd axelar-its-example
   ```

2. Build the contract:
   ```bash
   stellar contract build
   ```

3. Optimize the WASM file:
   ```bash
   stellar contract optimize --wasm target/wasm32-unknown-unknown/release/stellar_its_example.wasm
   ```

4. Deploy the contract to Stellar testnet:
   ```bash
   stellar contract deploy \
     --wasm target/wasm32-unknown-unknown/release/stellar_its_example.optimized.wasm \
     --source YOUR_ACCOUNT_NAME \
     --network testnet
   ```

## Usage

### 1. Initialize the Contract

```bash
stellar contract invoke \
  --network testnet \
  --id YOUR_CONTRACT_ADDRESS \
  --source-account YOUR_ACCOUNT_NAME \
  -- \
  initialize \
  --its_address CCXT3EAQ7GPQTJWENU62SIFBQ3D4JMNQSB77KRPTGBJ7ZWBYESZQBZRK
```

### 2. Check Trusted Chains

```bash
stellar contract invoke \
  --network testnet \
  --id YOUR_CONTRACT_ADDRESS \
  --source-account YOUR_ACCOUNT_NAME \
  -- \
  is_trusted_chain \
  --chain '"Avalanche"'
```

## New Interchain Token Operations

### 3. Deploy a New Token

```bash
stellar contract invoke \
  --network testnet \
  --id YOUR_CONTRACT_ADDRESS \
  --source-account YOUR_ACCOUNT_NAME \
  -- \
  deploy_token \
  --caller YOUR_ACCOUNT_NAME \
  --salt 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef \
  --name '"Your Token Name"' \
  --symbol '"TKN"' \
  --decimals 7 \
  --initial_supply 1000000000
```

### 4. Deploy the Token to Another Chain

```bash
stellar contract invoke \
  --network testnet \
  --id YOUR_CONTRACT_ADDRESS \
  --source-account YOUR_ACCOUNT_NAME \
  -- \
  deploy_remote_token \
  --caller YOUR_ACCOUNT_NAME \
  --destination_chain '"Avalanche"' \
  --gas_token_address CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
  --gas_amount 10000000
```

### 5. Transfer Tokens Cross-Chain

```bash
stellar contract invoke \
  --network testnet \
  --id YOUR_CONTRACT_ADDRESS \
  --source-account YOUR_ACCOUNT_NAME \
  -- \
  transfer_tokens \
  --caller YOUR_ACCOUNT_NAME \
  --token_id YOUR_TOKEN_ID \
  --destination_chain '"Avalanche"' \
  --destination_address '"YOUR_AVALANCHE_ADDRESS"' \
  --amount 1000000000 \
  --gas_token_address CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
  --gas_amount 10000000
```

## Existing (Canonical) Token Operations

### 1. Register an Existing Token

Register an existing Stellar token for cross-chain use:

```bash
stellar contract invoke \
  --network testnet \
  --id YOUR_CONTRACT_ADDRESS \
  --source-account YOUR_ACCOUNT_NAME \
  -- \
  register_existing_token \
  --caller YOUR_ACCOUNT_NAME \
  --token_address TOKEN_CONTRACT_ADDRESS
```

### 2. Deploy Existing Token to Another Chain

Deploy the registered token to another blockchain:

```bash
stellar contract invoke \
  --network testnet \
  --id YOUR_CONTRACT_ADDRESS \
  --source-account YOUR_ACCOUNT_NAME \
  -- \
  deploy_remote_canonical_token \
  --caller YOUR_ACCOUNT_NAME \
  --token_address TOKEN_CONTRACT_ADDRESS \
  --destination_chain '"Avalanche"' \
  --gas_token_address CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
  --gas_amount 10000000
```

### 3. Transfer Existing Tokens Cross-Chain

Transfer the registered existing tokens to another blockchain using the same `transfer_tokens` function with the canonical token ID:

```bash
stellar contract invoke \
  --network testnet \
  --id YOUR_CONTRACT_ADDRESS \
  --source-account YOUR_ACCOUNT_NAME \
  -- \
  transfer_tokens \
  --caller YOUR_ACCOUNT_NAME \
  --token_id YOUR_CANONICAL_TOKEN_ID \
  --destination_chain '"Avalanche"' \
  --destination_address '"YOUR_AVALANCHE_ADDRESS"' \
  --amount 100000000 \
  --gas_token_address CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
  --gas_amount 10000000
```

### 4. Get Current Token ID

Retrieve the ID of the last deployed or registered token:

```bash
stellar contract invoke \
  --network testnet \
  --id YOUR_CONTRACT_ADDRESS \
  --source-account YOUR_ACCOUNT_NAME \
  -- \
  get_token_id
```

## Contract Functions

### Common Functions
- `initialize`: Initialize the contract with the ITS address
- `is_trusted_chain`: Check if a chain is trusted
- `get_token_id`: Get the ID of the most recently deployed or registered token

### Native Interchain Token Functions
- `deploy_token`: Deploy a new token on Stellar
- `deploy_remote_token`: Deploy a token to another blockchain
- `transfer_tokens`: Transfer tokens to another blockchain (works for both native and canonical tokens)

### Existing (Canonical) Token Functions
- `register_existing_token`: Register an existing token for cross-chain use
- `deploy_remote_canonical_token`: Deploy a registered token to another blockchain

Working with Existing Tokens: Step-by-Step Guide

1. **Register your token**: Use `register_existing_token` to register an existing Stellar token with the ITS service.
2. **Deploy to destination chain** (optional): Use `deploy_remote_canonical_token` to make your token available on another blockchain.
3. **Transfer tokens**: Use `transfer_tokens` with the canonical token ID to send your tokens to the destination chain.

> **Note**: Unlike native interchain tokens, existing (canonical) tokens use the standard Stellar token interface. The ITS service handles the necessary token operations internally.

## Common Issues and Troubleshooting

### 1. Insufficient Token Balance
**Error**: `UnreachableCodeReached` when transferring tokens

**Solution**: Ensure you have sufficient balance of the token you're trying to transfer:
```bash
stellar contract invoke \
  --network testnet \
  --id TOKEN_CONTRACT_ADDRESS \
  --source-account YOUR_ACCOUNT_NAME \
  -- \
  balance \
  --id YOUR_ACCOUNT_NAME
```

### 2. Contract Not Deployed
**Error**: Contract not found or invalid address

**Solution**: Make sure you've deployed your wrapper contract and are using the correct contract address.

### 3. Gas Token Balance
**Error**: Insufficient gas for cross-chain transfers

**Solution**: Ensure you have enough of the gas token (typically native Stellar lumens for testnet):
```bash
stellar contract invoke \
  --network testnet \
  --id CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
  --source-account YOUR_ACCOUNT_NAME \
  -- \
  balance \
  --id YOUR_ACCOUNT_NAME
```

### 4. Token ID Issues
**Error**: Invalid token ID

**Solution**: 
- For native tokens: Use the token ID returned by `deploy_token`
- For canonical tokens: Use the token ID returned by `register_existing_token` or call `get_token_id` after registration

### 5. Chain Names
**Important**: Chain names are case-sensitive and must exactly match Axelar's configuration. Common names include:
- "Avalanche" (not "avalanche")
- "Ethereum" (not "ethereum")
- Always use quotes when passing chain names in CLI commands

## Tracking Cross-Chain Transactions

You can track your cross-chain transactions on:
- [Axelarscan Testnet](https://testnet.axelarscan.io)
- [Snowtrace (for Avalanche Fuji)](https://testnet.snowtrace.io)

## Resources

- [Axelar Documentation](https://docs.axelar.dev)
- [Stellar Documentation](https://developers.stellar.org/docs)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.