use super::core::{ext_lib::*, our_lib::*};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx : Context, _ready : Ready) {
        println!("{}", UTILS.events.ready_msg);
    }
}