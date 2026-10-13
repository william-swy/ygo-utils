use std::sync::Arc;
use tokio::{
    sync::Semaphore,
    task::JoinHandle,
    time::{Duration, interval},
};

pub struct Throttler {
    handle: JoinHandle<()>,
    semaphore: Arc<Semaphore>,
}

impl Throttler {
    pub fn new(requests_per_second: usize) -> Self {
        let semaphore = Arc::new(Semaphore::new(0));
        let sem_clone = Arc::clone(&semaphore);

        let handle = tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(1));

            loop {
                ticker.tick().await;

                let current_available = sem_clone.available_permits();
                if current_available < requests_per_second {
                    let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs();
                    println!("{} Replenished {}", current_time, requests_per_second - current_available);
                    sem_clone.add_permits(requests_per_second - current_available);
                }
            }
        });

        Self {
            handle: handle,
            semaphore: semaphore,
        }
    }

    pub async fn invoke(&self) {        
        let permit = self.semaphore.acquire().await.unwrap();
        permit.forget();
    }
}

impl Drop for Throttler {
    fn drop(&mut self) {
        self.handle.abort();
    }
}
