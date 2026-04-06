# 🧵 Vinculum-hs

**Seamless Haskell–Rust interoperability through automated FFI bindings and procedural macros.**

Write Haskell functions, call them from Rust with full type safety. Let Vinculum handle the rest.

[![Crates.io](https://img.shields.io/crates/v/vinculum-hs.svg)](https://crates.io/crates/vinculum-hs)
[![Docs.rs](https://docs.rs/vinculum-hs/badge.svg)](https://docs.rs/vinculum-hs)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## Overview

Vinculum is a Haskell–Rust interoperability framework that automates cross-language FFI bindings through compile-time code generation. Write your business logic in Haskell, use it from Rust without manual FFI plumbing or `unsafe` blocks.

Instead of:
```rust
// ❌ Manual FFI (error-prone, verbose)
extern "C" {
    fn hs_add(a: i64, b: i64) -> i64;
}
unsafe { hs_add(5, 10) }
```

You get:
```rust
// ✅ Type-safe, auto-generated wrappers
use vinculum::math::add;
add(5, 10)  // Type-safe, no unsafe
```

> *Vinculum* — Latin for "bond" or "link."

---

## Features

| Feature | Benefit |
|---------|---------|
| **Zero boilerplate** | FFI bindings generated automatically from Haskell signatures |
| **Type-safe** | Generated wrappers enforce correctness at compile time |
| **Single macro** | `#[vinculum::main]` handles Haskell RTS initialization/shutdown |
| **Transparent builds** | Haskell compilation orchestrated via `cargo build` |
| **Minimal overhead** | Direct FFI calls over the C ABI |
| **IDE-friendly** | Generated bindings enable autocomplete, type checking, and navigation |
| **Rich type support** | Int8-64, Word8-64, Float, Double, Bool, Char, String, Tuples |

---

## How It Works

```
┌────────────────────────────┐
│  Rust Application          │
│  use vinculum::math::add   │  ← Type-safe generated wrapper
├────────────────────────────┤
│  Code Generation (build)   │  ← Parse .hs → Generate Rust + Haskell
├────────────────────────────┤
│  Haskell Runtime (dylib)   │  ← GHC-compiled shared library
│  Dispatch → Real functions │
└────────────────────────────┘
```

### Build Pipeline

1. **Discovery** — Scan Haskell source directory for `.hs` files
2. **Parsing** — Extract function signatures and type annotations
3. **Haskell Codegen** — Generate `Dispatch.hs` and `UserFunctions.hs`
4. **Cabal Build** — Compile Haskell to `.dylib` / `.so` with FFI exports
5. **Rust Codegen** — Generate type-safe Rust wrappers
6. **Linking** — Cargo links Haskell library into binary

### Runtime Call Flow

```
Rust: add(5, 10)
   ↓ call_haskell_typed()
Encode: [Value::Int64(5), Value::Int64(10)] → bytes
   ↓ FFI boundary
Haskell Runtime.hs receives bytes
   ↓ dispatchUserFunction()
Haskell Math.add(5, 10) → 15
   ↓ encodeValue()
Decode bytes → Value::Int64(15)
   ↓
Result returned to Rust
```

---

## Quick Start

### Requirements

- **Rust** 1.93+
- **GHC** 9.0+ with Cabal
- macOS, Linux, or Windows

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
vinculum-hs = "0.1"

[build-dependencies]
vinculum-hs = "0.1"

[package.metadata]
cabal_file = "haskell/haskell.cabal"           # Path to your Cabal file
exports_dir = "haskell/app/exports"           # Where your .hs files are
foreign_library = "lib"                        # Foreign-library name (from .cabal)
```

Also create a `build.rs` in your project root:

```rust
fn main() {
    vinculum_hs::build().expect("Vinculum build failed");
}
```

### Running the Example

```bash
git clone https://github.com/enzoblain/Vinculum
cd Vinculum
cargo run --example math
```

---

## Example

### 1. Define Haskell Functions

**`examples/haskell/app/exports/Math.hs`**
```haskell
module Math where
import Data.Int

add :: Int64 -> Int64 -> Int64
add a b = a + b

multiply :: Int64 -> Int64 -> Int64
multiply a b = a * b

factorial :: Int64 -> Int64
factorial 0 = 1
factorial n = n * factorial (n - 1)
```

### 2. Configure in Rust Project

**`Cargo.toml`**
```toml
[package]
name = "my-app"

[dependencies]
vinculum-hs = "0.1"

[package.metadata]
haskell_directory = "haskell/app/exports"  # Where your .hs files are
```

**`build.rs`**
```rust
fn main() {
    vinculum_hs::build().expect("Vinculum build failed");
}
```

### 3. Use in Rust

**`src/main.rs`**
```rust
use vinculum::math::{add, multiply, factorial};

#[vinculum_hs::main]
fn main() {
    println!("5 + 10 = {}", add(5, 10));
    println!("5 * 10 = {}", multiply(5, 10));
    println!("Factorial 5 = {}", factorial(5));
}
```

**Output:**
```
5 + 10 = 15
5 * 10 = 50
Factorial 5 = 120
```

---

## Type Support

Vinculum supports a rich set of Haskell types:

| Haskell | Rust | Notes |
|---------|------|-------|
| `Int8` | `i8` | 8-bit signed integer |
| `Int16` | `i16` | 16-bit signed integer |
| `Int32` | `i32` | 32-bit signed integer |
| `Int64` | `i64` | 64-bit signed integer |
| `Word8` | `u8` | 8-bit unsigned integer |
| `Word16` | `u16` | 16-bit unsigned integer |
| `Word32` | `u32` | 32-bit unsigned integer |
| `Word64` | `u64` | 64-bit unsigned integer |
| `Float` | `f32` | Single-precision float |
| `Double` | `f64` | Double-precision float |
| `Bool` | `bool` | Boolean |
| `Char` | `char` | Unicode character |
| `String` | `String` | UTF-8 string |
| `(a, b)` | `(T1, T2)` | Tuples |

---

## Roadmap

- [x] Automatic FFI binding generation from Haskell modules
- [x] Procedural macro for RTS lifecycle
- [x] Type-safe cross-language wrappers
- [x] Cargo-driven build orchestration
- [x] Multi-type support (Int, Float, Bool, String, Tuples)
- [ ] `Maybe` and `Either` type support
- [ ] Async/concurrent interoperability
- [ ] Enhanced error propagation across FFI
- [ ] Benchmarks and performance optimization
- [ ] Extended documentation and examples

---

## Contributing

Contributions are welcome! Feel free to open an issue to discuss changes or submit a pull request.

---

## License

MIT

Please ensure that your contributions align with the project's goals and maintain code quality and consistency.

---

## License

This project is licensed under the MIT License.  
See the [LICENSE](LICENSE) file for details.