use std::{borrow::Cow, sync::Arc};
use tokio::sync::Mutex;

use rmcp::{RoleServer,service::RequestContext,
    handler::server::tool::ToolCallContext, model::{
        CallToolRequestParam, CallToolResult, Content, ErrorCode, Implementation, ListToolsResult, PaginatedRequestParam, ProtocolVersion, ServerCapabilities, ServerInfo
    }, tool, tool_router, transport::stdio, ErrorData, ServerHandler, ServiceExt,
};

#[derive(Clone)]
pub struct HelloWorld {
    counter: Arc<Mutex<i32>>,
}

#[tool_router]
impl HelloWorld {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    #[tool(name = "increment", description = "Tool that increments and decrements a counter")]
    async fn increment(&self) -> Result<CallToolResult, ErrorData> {
        let mut count = self.counter.lock().await;
        *count += 1;
        Ok(CallToolResult::success(vec![Content::text(
            count.to_string(),
        )]))
    }

    #[tool(name = "decrement", description = "Tool that decrements a counter")]
    async fn decrement(&self) -> Result<CallToolResult, ErrorData> {
        let mut count = self.counter.lock().await;
        *count -= 1;
        Ok(CallToolResult::success(vec![Content::text(
            count.to_string(),
        )]))
    }

    #[tool(name = "get_counter", description = "Tool that returns the current value of the counter")]
    async fn get_counter(&self) -> Result<CallToolResult, ErrorData> {
        let count = self.counter.lock().await;
        Ok(CallToolResult::success(vec![Content::text(
            count.to_string(),
        )]))
    }
}

impl ServerHandler for HelloWorld {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "This server provide counter tools that can increment, decrement, and retrieve the current value of a counter. Use the 'increment', 'decrement', and 'get_counter' tools to interact with the counter."
                    .to_string(),
            ),
        }
    }

    async fn list_tools(
        &self,
        _pagination: Option<PaginatedRequestParam>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        let tools = Self::tool_router().list_all();
        Ok(ListToolsResult {
            tools,
            next_cursor: None,
        })
    }
    async fn call_tool(
            &self,
            params: CallToolRequestParam,
            ctx: RequestContext<RoleServer>,
        ) -> Result<CallToolResult, ErrorData> {
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