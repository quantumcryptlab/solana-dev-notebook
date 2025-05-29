# Introduction to Solana Development

This tutorial covers the fundamentals of Solana blockchain development and helps you set up your development environment.

## Learning Objectives

- Understand Solana's core concepts
- Set up your development environment
- Create your first Solana program
- Learn about Solana's programming model

## Prerequisites

Before starting this tutorial, make sure you have:
- Basic understanding of blockchain concepts
- Familiarity with JavaScript/TypeScript
- Basic command line knowledge

## Development Environment Setup

1. Install Node.js (v14 or later)
```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 14
nvm use 14
```

2. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Install Solana CLI
```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
```

4. Install Anchor Framework
```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

## Solana Core Concepts

### 1. Account Model
- Accounts are like files in a filesystem
- All data is stored in accounts
- Accounts can store both data and programs

### 2. Programs (Smart Contracts)
- Programs are stateless
- They can read and write to accounts
- Programs are stored in accounts marked as executable

### 3. Transactions
- Atomic operations
- Can contain multiple instructions
- All instructions must succeed for the transaction to succeed

## Your First Solana Program

The `code` directory contains a simple "Hello World" program that demonstrates:
- Creating a basic Solana program
- Building and deploying to a local testnet
- Interacting with the program using JavaScript

### Running the Program

1. Navigate to the code directory:
```bash
cd code
```

2. Install dependencies:
```bash
npm install
```

3. Build the Rust program:
```bash
anchor build
```

4. Start a local test validator (in a new terminal):
```bash
solana-test-validator
```

5. Run the tests:
```bash
anchor test
```

The test will deploy the program to your local testnet and execute a transaction that prints "Hello, World!" to the program log.

## Next Steps

After completing this tutorial, you'll be ready to move on to [Accounts & Programs](../02-account-program) where you'll learn more about Solana's account system and how to create more complex programs.

## Additional Resources

- [Official Solana Documentation](https://docs.solana.com)
- [Solana Cookbook](https://solanacookbook.com)
- [Anchor Book](https://book.anchor-lang.com) 