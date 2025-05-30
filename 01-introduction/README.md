# Solana Fundamentals

This document covers the fundamental concepts of Solana blockchain development.

## Network Types

- **Mainnet**: The main Solana network where real transactions occur
- **Devnet**: Development network for testing with free SOL tokens
- **Testnet**: Test network for final testing before mainnet deployment

## Key Concepts

### Accounts and Tokens

- **Token Account**: An account that holds specific token balances
- **Mint Account**: The account that controls token creation and supply
- **Token 2022**: Upgraded token standard with enhanced features
- **Associated Token Account (ATA)**: 
  - A unique account created for each token type a user owns
  - Requires SOL for rent payment
  - Can be closed to reclaim rent when no longer needed

### Program Derived Address (PDA)
- Addresses without private keys
- Used for:
  - Program authority over other accounts
  - Signing transactions
  - Data storage
- Deterministically generated from seeds

### Currency Units
- **Lamports**: Smallest unit of SOL (like wei in Ethereum)
- 1 SOL = 1,000,000,000 lamports

### Transaction Fees
- **Priority Fees**: 
  - Additional SOL paid to validators
  - Higher fees = faster transaction processing
  - Dynamic based on network congestion

### Account Rent
- SOL payment required for storing data/code on-chain
- Based on account size
- Can be reclaimed by closing the account
- Rent-exempt accounts possible with sufficient SOL deposit

## Development Tools
- Solana CLI
- Anchor Framework
- Rust Programming Language
- Web3.js/TypeScript SDK

## Common Operations
- Account Creation
- Token Management
- Program Deployment
- Cross-Program Invocation (CPI)
- Transaction Signing
- State Management

## Best Practices
- Always check account ownership
- Validate all inputs
- Handle rent collection properly
- Use PDAs for program-controlled accounts
- Implement proper error handling
- Follow security guidelines

## Resources
- [Solana Documentation](https://docs.solana.com)
- [Anchor Framework Docs](https://www.anchor-lang.com)
- [Solana Cookbook](https://solanacookbook.com)
- [Token Program](https://spl.solana.com/token)

