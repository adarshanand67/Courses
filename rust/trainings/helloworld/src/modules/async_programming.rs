use tokio::time::{Duration, sleep};

pub async fn async_example() {
    println!("Starting async task...");
    sleep(Duration::from_secs(2)).await;
    println!("Async task completed!");
}

pub fn async_programming_example() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async_example());
}
