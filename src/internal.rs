use std::{sync::atomic::{AtomicUsize, Ordering}, time::{SystemTime, UNIX_EPOCH}};

use crate::{errors::LimitExceeded, Snowflake};

pub(crate) const MAX_5_BITS: usize = 31;
const MAX_12_BITS: usize = 4095;

pub struct SnowNode {
    pub id: usize,
    base_epoch: u64,
    workers: Vec<SnowWorker>,
}

pub struct SnowWorker {
    id: usize,
    node_id: usize,
    base_epoch: u64,
    sequence: AtomicUsize,
}

impl SnowNode {
    pub fn new(id: usize, base_epoch: u64) -> Self {
        Self {
            id,
            base_epoch,
            workers: Vec::with_capacity(MAX_5_BITS)
        }
    }

    pub fn new_worker(&mut self) -> Result<&mut SnowWorker, LimitExceeded> {
        let workers_len = self.workers.len();

        if workers_len >= MAX_5_BITS {
            return Err(LimitExceeded)
        }

        self.workers.push(
            SnowWorker::new(workers_len, self.id, self.base_epoch)
        );

        Ok(self.workers.last_mut().unwrap())
    }
}

impl SnowWorker {
    pub fn new(id: usize, node_id: usize, base_epoch: u64) -> Self {
        Self {
            id,
            node_id,
            base_epoch,
            sequence: AtomicUsize::new(0),
        }
    }

    pub fn generate(&self) -> Result<Snowflake, LimitExceeded> {
        let increment = match self.increment() {
            Ok(value) => value,
            Err(e) => return Err(e)
        };

        let snowflake_id = self.get_timestamp() << 22
            | self.node_id << 17
            | self.id << 12
            | increment;

        Ok(Snowflake::new(snowflake_id as u64))
    }

    fn get_timestamp(&self) -> usize {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| ()).unwrap()
            .as_millis();

        (current_time - self.base_epoch as u128) as usize
    }

    fn increment(&self) -> Result<usize, LimitExceeded> {
        let current = self.sequence.load(Ordering::SeqCst);

        if current >= MAX_12_BITS {
            return Err(LimitExceeded);
        }

        self.sequence.store(current + 1, Ordering::SeqCst);
        Ok(current)
    }
}