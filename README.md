# MCP IMPLEMENTATION IN RUST - TUTORIAL by Chatelo.

## Step 1: Project Setup

Let's start by creating a new Rust project. In your terminal, run:

```
cargo new rust_counter_mcp
```

This will create a new folder with the basic Rust project structure. Now, let's edit the `Cargo.toml` file to add the dependencies we'll need:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
rmcp = "0.3"
```

**What do these dependencies do?**
- `tokio`: This is an asynchronous runtime for Rust. It allows us to write async code, which is important for servers that handle multiple requests at once. [Tokio docs](https://docs.rs/tokio/latest/tokio/)
- `rmcp`: This is the official Rust implementation of the Model Context Protocol (MCP), which lets us build AI assistant servers and tools. [rmcp docs](https://docs.rs/rmcp/latest/rmcp/)

You can read more about dependencies in the [Cargo Book](https://doc.rust-lang.org/cargo/).

Now, run:
```
cargo build
```
This will download and compile the dependencies.

---
**Next step:** We'll write a minimal "Hello World" in Rust and explain every line.

# Step 2: Writing a minimal "Hello World" in Rust

Let's start with the simplest Rust program. Open `src/main.rs` and write:

```rust
fn main() {
    println!("Hello, world!");
}
```
- `fn main() { ... }` defines the main function, which is the entry point of every Rust program.
- `println!(...)` is a macro that prints text to the console. The `!` means it's a macro, not a regular function. [println! macro docs](https://doc.rust-lang.org/std/macro.println.html)

To run this program, use:
```
cargo run
```
You should see `Hello, world!` printed in your terminal.

---
**Next step:** We'll add async support with Tokio and explain what async means in Rust.

# Step 3: Adding async support with Tokio

Rust supports asynchronous programming, which lets your program handle many tasks at once without blocking. This is especially useful for servers.

To use async in Rust, we need an async runtime. Tokio is the most popular one. Let's update our `main.rs`:

```rust
use tokio; // Import the Tokio crate

#[tokio::main]
async fn main() {
    println!("Hello, async world!");
}
```
```
- `use tokio;` brings the Tokio crate into scope. (Tokio docs: https://docs.rs/tokio/latest/tokio/)
- `#[tokio::main]` is a macro that sets up the Tokio runtime. It allows us to use `async fn main()`.
- `async fn main()` means our main function is asynchronous. We can now use `.await` inside it.
```

To run this, use:
```
cargo run
```
You should see `Hello, async world!` printed.

---
**Next step:** We'll introduce the MCP protocol and the `rmcp` crate, explaining what they do and how to use them.

# Step 4: Introducing the MCP protocol and the `rmcp` crate

Now let's talk about the Model Context Protocol (MCP) and the `rmcp` crate.

**What is MCP?**
- MCP is a protocol that lets AI assistants (like Claude or Cursor IDE) communicate with other services. It defines how tools and servers talk to each other.
- The official Rust implementation is the `rmcp` crate. [rmcp docs](https://docs.rs/rmcp/latest/rmcp/)

**What does `rmcp` do?**
- It provides macros, types, and traits to build servers and tools that follow the MCP protocol.
- You can use it to create tools (functions that do something) and expose them to clients.

**How does it fit in our project?**
- We'll use `rmcp` to build a server that exposes a simple counter tool. Clients can connect and use our tools.

---
**Next step:** We'll build a simple MCP server with a counter tool, starting from the struct definition and explaining every line.

# Step 5: Building a simple MCP server with a counter tool

Let's start building our MCP server. We'll begin by defining a struct to hold our counter.

Open `src/main.rs` and add:

```rust
use std::sync::Arc; // Arc lets us share data safely between threads
use tokio::sync::Mutex; // Mutex lets us safely mutate data in async code

#[derive(Clone)]
pub struct HelloWorld {
    counter: Arc<Mutex<i32>>, // Shared, mutable counter
}

impl HelloWorld {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)), // Start counter at 0
        }
    }
}
```

**Explanation:**
- `use std::sync::Arc;` brings in the Arc type from the standard library. [Arc docs](https://doc.rust-lang.org/std/sync/struct.Arc.html)
- `use tokio::sync::Mutex;` brings in the async Mutex from Tokio. [Mutex docs](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html)
- `#[derive(Clone)]` automatically implements the Clone trait, so we can duplicate our struct safely.
- `pub struct HelloWorld { ... }` defines our server struct, which holds the counter.
- `Arc<Mutex<i32>>` means our counter is an integer, wrapped in an Arc (for sharing) and a Mutex (for safe mutation).
- `pub fn new() -> Self { ... }` is a constructor that initializes the counter to 0.

---
**Next step:** We'll explain every struct, trait, macro, and function used so far, and then add the MCP tool macros to expose our counter functions.

# Step 6: Explaining every struct, trait, macro, and function used

Let's break down what we've used so far:

- **Structs:**
  - `HelloWorld`: Our main server struct. In Rust, a struct is a way to group related data together. [Struct docs](https://doc.rust-lang.org/std/keyword.struct.html)
  - `Arc<T>`: Stands for "Atomic Reference Counted". It lets us share data between threads safely. [Arc docs](https://doc.rust-lang.org/std/sync/struct.Arc.html)
  - `Mutex<T>`: A mutual exclusion primitive for protecting shared data. Tokio's Mutex works with async code. [Mutex docs](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html)

- **Traits:**
  - `Clone`: Lets us duplicate our struct. The `#[derive(Clone)]` macro automatically implements this trait. [Clone docs](https://doc.rust-lang.org/std/clone/trait.Clone.html)

- **Functions:**
  - `new()`: A constructor for our struct. Returns a new instance with the counter set to 0.

- **Macros:**
  - `#[derive(Clone)]`: Automatically generates code to implement the Clone trait.

Next, let's add MCP tool macros to expose our counter functions. We'll use the `#[tool_router]` and `#[tool]` macros from the `rmcp` crate.

---
**Next step:** We'll detail concurrency primitives (`Arc`, `Mutex`) and add the MCP tool macros to implement increment, decrement, and get_counter tools.
# Step 7: Detailing concurrency primitives (`Arc`, `Mutex`)

Concurrency means running multiple tasks at the same time. In Rust, we use special types to safely share and mutate data between threads or async tasks:

- **Arc (Atomic Reference Counted):**
  - Lets multiple parts of your program share ownership of data.
  - Automatically keeps track of how many references exist, and cleans up when none are left.
  - [Arc docs](https://doc.rust-lang.org/std/sync/struct.Arc.html)

- **Mutex (Mutual Exclusion):**
  - Ensures only one part of your program can access the data at a time.
  - Tokio's Mutex works with async code, so you can `.await` when locking.
  - [Mutex docs](https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html)

Example:
```rust
let counter = Arc::new(Mutex::new(0));
```
This means `counter` is an integer, shared and protected for safe concurrent access.

---
# Step 8: Implementing and explaining each tool (increment, decrement, get_counter)

Now let's expose our counter functions as MCP tools using the `rmcp` macros.

Add this to your `HelloWorld` implementation:

```rust
use rmcp::{tool, tool_router, model::{CallToolResult, Content}, ErrorData};

#[tool_router]
impl HelloWorld {
    #[tool(name = "increment", description = "Tool that increments the counter")]
    async fn increment(&self) -> Result<CallToolResult, ErrorData> {
        let mut count = self.counter.lock().await;
        *count += 1;
        Ok(CallToolResult::success(vec![Content::text(count.to_string())]))
    }

    #[tool(name = "decrement", description = "Tool that decrements the counter")]
    async fn decrement(&self) -> Result<CallToolResult, ErrorData> {
        let mut count = self.counter.lock().await;
        *count -= 1;
        Ok(CallToolResult::success(vec![Content::text(count.to_string())]))
    }

    #[tool(name = "get_counter", description = "Tool that returns the current value of the counter")]
    async fn get_counter(&self) -> Result<CallToolResult, ErrorData> {
        let count = self.counter.lock().await;
        Ok(CallToolResult::success(vec![Content::text(count.to_string())]))
    }
}
```

**Explanation:**
- `#[tool_router]` macro marks this impl block as containing MCP tools.
- `#[tool(...)]` macro exposes each function as a tool to clients.
- Each function locks the counter, updates or reads it, and returns the result as text.
- `CallToolResult::success(...)` wraps the result for MCP clients.

---
**Next step:** We'll implement the server handler and main function, explaining every line.
# Step 9: Implementing the server handler and main function

To make our server work with MCP, we need to implement the `ServerHandler` trait from `rmcp`. This trait tells MCP how to interact with our tools.

Add this to your `main.rs`:

```rust
use rmcp::{RoleServer, model::{ServerInfo, ProtocolVersion, ServerCapabilities, Implementation, PaginatedRequestParam, ListToolsResult, CallToolRequestParam, CallToolResult}, handler::server::tool::ToolCallContext, ErrorData, ServerHandler, ServiceExt, transport::stdio, service::RequestContext};

impl ServerHandler for HelloWorld {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides counter tools that can increment, decrement, and retrieve the current value of a counter. Use the 'increment', 'decrement', and 'get_counter' tools to interact with the counter.".to_string()),
        }
    }

    async fn list_tools(&self, _pagination: Option<PaginatedRequestParam>, _ctx: RequestContext<RoleServer>) -> Result<ListToolsResult, ErrorData> {
        let tools = Self::tool_router().list_all();
        Ok(ListToolsResult { tools, next_cursor: None })
    }

    async fn call_tool(&self, params: CallToolRequestParam, ctx: RequestContext<RoleServer>) -> Result<CallToolResult, ErrorData> {
        let context = ToolCallContext {
            request_context: ctx,
            service: self,
            name: params.name,
            arguments: params.arguments,
        };
        Self::tool_router().call(context).await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = HelloWorld::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
```

**Explanation:**
- `impl ServerHandler for HelloWorld { ... }` implements the MCP server trait for our struct.
- `get_info` returns metadata about our server.
- `list_tools` returns the list of available tools.
- `call_tool` dispatches tool calls to the right function.
- `#[tokio::main] async fn main()` starts the server and waits for requests.
- `HelloWorld::new().serve(stdio()).await?;` starts the MCP server using standard input/output.

---
# Step 10: Running and testing the server

Now let's run our MCP server!

In your terminal, run:
```
cargo run
```

If everything is set up correctly, your server will start and wait for requests. You can connect to it using an MCP-compatible client (like Claude Desktop, Cursor IDE, or your own client using the `rmcp` crate).

**How to interact with the server:**
- The server exposes three tools: `increment`, `decrement`, and `get_counter`.
- Clients can call these tools to change or read the counter value.
- You can build a client in Rust using the `rmcp` crate, or use other MCP-compatible tools.

**Troubleshooting:**
- If you see errors, check that your `Cargo.toml` has the correct dependencies and that your code matches the tutorial.
- Refer to the [rmcp docs](https://docs.rs/rmcp/latest/rmcp/) and [Rust std docs](https://doc.rust-lang.org/stable/std/index.html) for more help.

---
# Step 11: Final thoughts and further reading

Congratulations! You've built a simple MCP server in Rust from scratch, learned about async programming, concurrency, and how to expose tools to clients.

**Where to go next:**
- Explore the [rmcp documentation](https://docs.rs/rmcp/latest/rmcp/) for more advanced features.
- Learn more about Rust's standard library: [Rust std docs](https://doc.rust-lang.org/stable/std/index.html)
- Try building your own tools and exposing them via MCP.
- Experiment with clients that can connect to your server.

**Recommended reading:**
- [The Rust Book](https://doc.rust-lang.org/book/): The best resource for learning Rust.
- [Tokio async runtime](https://docs.rs/tokio/latest/tokio/): Learn more about async programming in Rust.
- [Cargo Book](https://doc.rust-lang.org/cargo/): Learn about Rust's package manager and build system.

If you get stuck, search the official docs or ask for help in the Rust community (Discord, forums, etc.).

---
**You now have a complete beginner-friendly tutorial for building an MCP server in Rust!**
