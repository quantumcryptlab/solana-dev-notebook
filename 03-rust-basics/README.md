# Rust Hello World Tutorial

This tutorial provides a basic introduction to Rust programming language. We'll cover essential concepts and basic syntax.

## Project Setup

1. Create a new Rust project:
```bash
cargo new rust-hello-world
cd rust-hello-world
```

2. Common Cargo Commands:
- `cargo check`: Check code for errors without building
- `cargo build`: Compile the project
- `cargo run`: Build and run the project
- `cargo test`: Run tests
- `cargo doc`: Generate documentation

## Basic Concepts

### Variables and Mutability
- By default, variables are immutable in Rust
- Use `mut` keyword to make variables mutable:
```rust
let x = 5; // immutable
let mut y = 5; // mutable
```

### Common Data Types

1. **Integer Types**
   - Default is i32 (32-bit)
   - Signed: i8, i16, i32, i64, i128
   - Unsigned: u8, u16, u32, u64, u128

2. **Floating-Point Types**
   - Default is f64 (64-bit)
   - Types: f32, f64

3. **Boolean Type**
   - Values: true, false
   - Size: 1 byte

4. **Character Type**
   - Unicode scalar value
   - 4 bytes (32-bit)
   - Supports emoji and international characters

5. **Compound Types**
   - **Tuples**: Fixed-length collection of values of different types
   ```rust
   let tup: (i32, f64, bool) = (500, 6.4, true);
   ```
   
   - **Arrays**: Fixed-length collection of same type
   ```rust
   let arr: [i32; 5] = [1, 2, 3, 4, 5];
   ```

### Functions
```rust
fn function_name(parameter: Type) -> ReturnType {
    // function body
}
```

### Ownership
One of Rust's most unique features:
- Each value has a single owner
- Only one owner at a time
- Value is dropped when owner goes out of scope
- Prevents memory leaks and ensures memory safety

Example:
```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved to s2, s1 is no longer valid
```

For simple types (integers, booleans, etc.), Rust automatically makes copies instead of moving ownership. These types implement the `Copy` trait.

### Structs and Enums
Structs: Custom data types to group related data
```rust
struct User {
    username: String,
    email: String,
    active: bool,
}
```

Enums: Type that can be one of several variants
```rust
enum IpAddrKind {
    V4,
    V6,
}
```

## Learning Resources
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Tour of Rust](https://tourofrust.com/)
- [Rust Playground](https://play.rust-lang.org/)

## Next Steps
- Error handling with Result and Option
- Pattern matching with match expressions
- Collections (Vec, HashMap)
- Modules and packages
- Traits and generics