use std::sync::Arc;

use crate::app::traits::Processor;

use anyhow::Result;
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub trait Listener {
    async fn listen<T>(&mut self, obj: &mut Arc<Mutex<T>>) -> Result<()>
    where
        T: Processor + Send;
}
