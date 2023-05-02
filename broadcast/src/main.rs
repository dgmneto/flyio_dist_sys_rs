use async_trait::async_trait;
use maelstrom::protocol::Message;
use maelstrom::{done, Node, Result, Runtime};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock};

pub fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let handler = Arc::new(Handler::default());
    Runtime::new().with_handler(handler).run().await
}

#[derive(Default)]
struct State {
    by_node: Arc<Mutex<HashMap<String, HashSet<u64>>>>,
    global: Arc<RwLock<HashSet<u64>>>,
}

impl State {
    fn try_add(&self, node: &str, val: u64) -> bool {
        if self.global.read().unwrap().contains(&val) {
            return false;
        }
        let mut global = self.global.write().unwrap();
        if global.contains(&val) {
            return false;
        }
        let mut by_node = self.by_node.lock().unwrap();
        global.insert(val);
        if by_node.contains_key(node) {
            by_node.get_mut(node).unwrap().insert(val);
        } else {
            by_node.insert(node.to_string(), vec![val].into_iter().collect());
        }
        true
    }

    fn all(&self) -> Vec<u64> {
        self.global.read().unwrap().clone().into_iter().collect()
    }
}

#[derive(Default)]
struct Handler {
    state: State,
}

#[async_trait]
impl Node for Handler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        let body: Broadcast = req.body.as_obj()?;
        match body {
            Broadcast::Broadcast { message } => {
                if self.state.try_add(runtime.node_id(), message) {
                    for node in runtime.nodes() {
                        if node == runtime.node_id() {
                            continue;
                        }
                        runtime.send(node, Broadcast::BroadcastInner { message }).await?;
                    }
                }
                runtime.reply_ok(req).await
            }
            Broadcast::BroadcastInner { message } => {
                self.state.try_add(&req.src, message);
                runtime.reply_ok(req).await
            }
            Broadcast::BroadcastInnerOk {} => {
                Ok(())
            }
            Broadcast::Read {} => {
                let reply = Broadcast::ReadOk {
                    messages: self.state.all(),
                };
                runtime.reply(req, reply).await
            }
            Broadcast::Topology { .. } => {
                runtime.reply_ok(req).await
            }
            _ => done(runtime, req),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum Broadcast {
    Init {},
    Broadcast {
        message: u64,
    },
    BroadcastInner {
        message: u64,
    },
    BroadcastInnerOk {},
    Read {},
    ReadOk {
        messages: Vec<u64>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
}
