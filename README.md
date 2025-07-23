# Rust Counter MCP

This project implements a simple Model Context Protocol (MCP) server in Rust that exposes a set of counter tools via the MCP protocol. The server allows clients to increment, decrement, and retrieve the value of a shared counter using defined tools.

## Features
- **Increment Tool**: Increases the counter by 1.
- **Decrement Tool**: Decreases the counter by 1.
- **Get Counter Tool**: Returns the current value of the counter.
- **MCP Protocol**: Implements the MCP server protocol for tool-based interaction.

## Code Overview
- The main logic is in `src/main.rs`.
- The `HelloWorld` struct manages the counter state using an async mutex for safe concurrent access.
- Tools are defined using the `#[tool]` macro and exposed via the MCP protocol.
- The server is started using Tokio and listens for requests over stdio.

## Usage

### Build
Make sure you have Rust and Cargo installed. Then run:

```zsh
cargo build --release
```

### Run
Start the MCP server:

```zsh
cargo run --release
```

The server will listen for MCP requests over stdio.

### Example Tool Calls
- **Increment Counter**:
  - Tool name: `increment`
  - Description: Increments the counter by 1 and returns the new value.
- **Decrement Counter**:
  - Tool name: `decrement`
  - Description: Decrements the counter by 1 and returns the new value.
- **Get Counter Value**:
  - Tool name: `get_counter`
  - Description: Returns the current value of the counter.

## Dependencies
- [tokio](https://crates.io/crates/tokio) for async runtime
- [rmcp](https://crates.io/crates/rmcp) for MCP protocol implementation

## Project Structure
- `src/main.rs`: Main server implementation
- `Cargo.toml`: Rust dependencies and metadata

## Notes
- The server is designed for demonstration and testing of MCP tool capabilities.
- You can extend the toolset by adding more methods to the `HelloWorld` struct and annotating them with the `#[tool]` macro.

## License
MIT
