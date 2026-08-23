# Vinculum

[![Crates.io](https://img.shields.io/crates/v/vinculum-main.svg)](https://crates.io/crates/vinculum-main)
[![Docs.rs](https://docs.rs/vinculum-main/badge.svg)](https://docs.rs/vinculum-main)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://img.shields.io/github/actions/workflow/status/enzoblain/vinculum/ci.yml?branch=main)](https://github.com/enzoblain/vinculum/actions)

**Vinculum** is the core framework for building **type-safe bridges between Rust and other languages**.

It provides the common infrastructure required to generate Rust binding files, serialize values, validate types, and establish communication with language-specific backends.

Language integrations are provided through dedicated crates such as `vinculum-jl`, `vinculum-py`, or `vinculum-hs`.

## Overview

`vinculum-main` is the language-independent foundation of the Vinculum ecosystem.

It defines the common types, serialization protocol, conversion traits, and code generation infrastructure shared by every backend. Language-specific crates are responsible for connecting this foundation to their respective runtimes and generating the Rust-side bindings.

```text
                         Vinculum
                            │
              ┌─────────────┴─────────────┐
              │       vinculum-main       │
              │                           │
              │ Types · Serialization     │
              │ Validation · Codegen      │
              └─────────────┬─────────────┘
                            │
          ┌─────────────────┼─────────────────┐
          │                 │                 │
   vinculum-jl       vinculum-py       vinculum-hs
      Julia             Python            Haskell
```

Each backend uses the same core abstractions while providing the runtime-specific implementation required to communicate with the target language.

## Features

* Shared foundation for all Vinculum backends.
* Automatic Rust binding generation.
* Strongly typed cross-language interfaces.
* Compile-time type validation.
* Deterministic binary serialization.
* Backend-independent value representation.
* Extensible architecture for new languages.

## Architecture

The core provides the protocol used to move typed values between Rust and a foreign runtime.

```text
Rust Application
       │
       ▼
Generated Rust Bindings
       │
       ▼
vinculum-main
       │
       ▼
Language Backend
       │
       ▼
Foreign Runtime
```

`vinculum-main` does not target a specific language. Its purpose is to provide the foundation that every `vinculum-*` backend builds upon.

## Ecosystem

| Crate           | Description                                               |
| --------------- | --------------------------------------------------------- |
| `vinculum-main` | Core types, serialization, validation and code generation |
| `vinculum-jl`   | Julia ↔ Rust integration                                  |
| `vinculum-*`    | Language-specific integrations                            |

## Contributing

Contributions are welcome. For significant changes, please open an issue before submitting a pull request.

## License

Licensed under the MIT License.
