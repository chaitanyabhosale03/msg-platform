use crate::network::protocol::Message;

pub struct MessageQueue {}

impl MessageQueue {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn enqueue(&self, _recipient: &str, _msg: Message) -> anyhow::Result<()> {
        // TODO: Store encrypted message in database
        Ok(())
    }

    pub async fn dequeue(&self, _recipient: &str) -> anyhow::Result<Vec<Message>> {
        // TODO: Fetch and delete messages from database
        Ok(vec![])
    }
}
