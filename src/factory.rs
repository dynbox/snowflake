use crate::{errors::LimitExceeded, internal::{SnowNode, MAX_5_BITS}};

pub struct SnowflakeSettings {
    base_epoch: u64,
}

impl Default for SnowflakeSettings {
    fn default() -> Self {
        Self {
            base_epoch: 1420070400000,
        }
    }
}

pub struct SnowflakeManager {
    settings: SnowflakeSettings,
    nodes: Vec<SnowNode>,
}

impl SnowflakeManager {
    pub fn new(settings: SnowflakeSettings) -> Self {
        Self {
            settings,
            nodes: Vec::with_capacity(MAX_5_BITS)
        }
    }

    pub fn new_node(&mut self) -> Result<&mut SnowNode, LimitExceeded> {
        let nodes_len = self.nodes.len();

        if nodes_len >= MAX_5_BITS {
            return Err(LimitExceeded)
        }

        self.nodes.push(SnowNode::new(nodes_len, self.settings.base_epoch));

        Ok(self.nodes.last_mut().unwrap())
    }
}