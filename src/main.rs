use rand::{Fill as _, thread_rng};
use std::time::{Duration, Instant};

struct Profile {
    sum: i64,
    duration: Duration,
}

fn profile(values: Vec<i32>) -> Profile {
    let mut sum = 0i64;
    let now = Instant::now();
    for value in values {
        if value > 40_000 {
            sum += value as i64;
        }
    }
    Profile { sum, duration: now.elapsed() }
}

fn main() {
    let mut values = vec![0; 100_000_000];
    values.try_fill(&mut thread_rng()).unwrap();
    println!("{:?}...", &values[..4]);
    let unsorted = profile(values.clone());
    values.sort();
    let sorted = profile(values);
    assert_eq!(unsorted.sum, sorted.sum);
    println!("unsorted: {}ms", unsorted.duration.as_millis());
    println!("sorted: {}ms", sorted.duration.as_millis());
}
