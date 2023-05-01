use async_trait::async_trait;
use maelstrom::protocol::{Message};
use maelstrom::{done, Node, Result, Runtime};
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use serde::{Deserialize, Serialize};

pub fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let handler = Arc::new(Handler::default());
    Runtime::new().with_handler(handler).run().await
}

#[derive(Default)]
struct Handler {
    id: AtomicU64,
}

#[async_trait]
impl Node for Handler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        if req.get_type() == "generate" {
            let id = self.id.fetch_add(1, Ordering::SeqCst);
            let reply = MyMessage::GenerateOk { id: (format!("{}-{}", runtime.node_id(), id)) };
            
            return runtime.reply(req, reply).await;
        }
        done(runtime, req)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum MyMessage {
    GenerateOk{
        id: String,
    },
}