use std::thread::sleep;
use std::time::{Duration, Instant};

use rand::Rng;

pub fn dtf<O>(min_duration: Duration, jitter: Duration, f: impl Fn() -> O) -> O {
    let start = Instant::now();
    let mut rng = rand::rng();
    let jitter_value = rng.random_range(0..jitter.as_millis() as u64);
    let jitter = Duration::from_millis(jitter_value);
    let result = f();
    let elapsed = start.elapsed();
    let wait_time = (min_duration - elapsed).max(Duration::ZERO) + jitter;
    sleep(wait_time);
    result
}
