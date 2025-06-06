# Number Guessing Game

A simple command-line number guessing game written in Rust. This project demonstrates basic Rust programming concepts including user input, random number generation, and control flow.

## Game Rules

The program will:
1. Generate a random number between 1 and 100
2. Ask the player to guess the number
3. Provide feedback if the guess is too high or too low
4. Keep track of the number of attempts
5. Congratulate the player when they guess correctly

## Technical Features

- Input validation and error handling
- Use of the `rand` crate for random number generation
- Pattern matching with `match` expressions
- Loop control with `loop` and `break`
- Basic type conversion and error handling

## Prerequisites

- Rust programming language (latest stable version)
- Cargo package manager

## Installation

1. Clone the repository or create a new directory
2. Ensure you have Rust and Cargo installed
3. Build the project:
```bash
cargo build
```

## Running the Game

To start the game, run:
```bash
cargo run
```

## Project Structure

- `src/main.rs`: Main game logic
- `Cargo.toml`: Project dependencies and metadata

## Dependencies

- `rand = "0.8.5"`: Random number generation

## Learning Points

This project demonstrates several Rust concepts:
- Variables and mutability
- Basic error handling with `match`
- User input processing
- Type conversion
- Control flow with loops and pattern matching
- External crate usage

## Future Improvements

Potential enhancements could include:
- Difficulty levels
- Score tracking
- Multiple rounds
- Time limits
- High score system

## License

This project is open source and available under the MIT License.

```bash
cargo new guess_game
```

生成一个随机数

引入生成随机数的包 rand
crates.io 网址 https://crates.io/crates/rand

获取键盘输入

使用标准库中的std::io


