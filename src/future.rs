use std::time::{Duration, Instant};

use rand::Rng;
use tokio::time::sleep;

pub async fn dtf<F, O>(min_duration: Duration, jitter: Duration, f: F) -> O
where
    F: Future<Output = O> + 'static,
{
    let start = Instant::now();
    let mut rng = rand::rng();
    let jitter_value = rng.random_range(0..jitter.as_millis() as u64);
    let jitter = Duration::from_millis(jitter_value);
    let result = f.await;
    let elapsed = start.elapsed();
    let wait_time = (min_duration - elapsed).max(Duration::ZERO) + jitter;
    sleep(wait_time).await;
    result
}
