use std::{sync::Arc, time::Duration};

use chrono::{NaiveDateTime, Utc};
use starduck::traits::UpdateStateFrom;
use tokio::sync::Mutex;
use tokio::time::sleep;

use anyhow::Result;

use starduck::traits::WithOffset;
use starduck::utils;
use starduck::utils::TIMEOUT_CHECK;

use crate::app::traits::Processor;

const DEFAULT_TIMEOUT_CHECK: u64 = 10;

pub async fn clock<T>(obj: &mut Arc<Mutex<T>>) -> Result<()>
where
    T: Processor + UpdateStateFrom<NaiveDateTime>,
{
    let timeout_check = utils::get(TIMEOUT_CHECK).unwrap_or_else(|e| {
        error!("Failed to fetch timeout check value: {e}");
        warn!("Defaulting timeout_check to {DEFAULT_TIMEOUT_CHECK}");

        DEFAULT_TIMEOUT_CHECK
    });
    let timeout_duration = Duration::from_secs(timeout_check);

    loop {
        sleep(timeout_duration.clone()).await;
        let now = Utc::now_with_offset();

        info!("Timeout check at {}", &now);

        let mut guard = obj.lock().await;

        match guard.update_state_from(now) {
            Ok(_) => {
                info!("Timeout check complete");
                drop(guard);
            }
            Err(e) => error!("Failed to complete timeout check: {}", e),
        };
    }
}
