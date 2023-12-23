use crate::PartitionHandle;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use std_semaphore::Semaphore;

pub struct CompactionManagerInner {
    partitions: Mutex<VecDeque<PartitionHandle>>,
    semaphore: Semaphore,
}

impl Default for CompactionManagerInner {
    fn default() -> Self {
        Self {
            partitions: Mutex::new(VecDeque::with_capacity(10)),
            semaphore: Semaphore::new(0),
        }
    }
}

#[derive(Clone, Default)]
pub struct CompactionManager(Arc<CompactionManagerInner>);

impl std::ops::Deref for CompactionManager {
    type Target = CompactionManagerInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CompactionManager {
    pub fn wait_for(&self) {
        self.semaphore.acquire();
    }

    pub fn notify(&self, partition: PartitionHandle) {
        let mut lock = self.partitions.lock().expect("lock is poisoned");
        lock.push_back(partition);
        self.semaphore.release();
    }

    pub fn pop(&self) -> Option<PartitionHandle> {
        let mut lock = self.partitions.lock().expect("lock is poisoned");
        lock.pop_front()
    }
}
