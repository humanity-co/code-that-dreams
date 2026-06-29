# Code That Dreams

A self-evolving Rust simulation platform utilizing an embedded scripting interpreter to dynamically generate, mutate, and run code execution scripts.

## Technical Architecture
* **Execution Core:** Integrates the Rhai scripting engine to run dynamic, non-compiled scripts within a secure sandbox environment.
* **Mutation Logic:** Evaluates past script outputs to apply syntactic mutations, checking for loop completion and processing output.
* **Asynchronous Driver:** Driven by Tokio to schedule script timeouts and handle execution crashes gracefully.

## Repository Layout
* `/src` - Rust simulator engine, script sandbox executor, and mutation handlers.
* `Cargo.toml` - Dependency declarations for Tokiox, Rhai, and Serde.

## Setup and Installation
1. Compile the simulator:
   ```bash
   cargo build --release
   ```
2. Launch the scripting lifecycle loop:
   ```bash
   cargo run
   ```

## License
Proprietary. All rights reserved.
