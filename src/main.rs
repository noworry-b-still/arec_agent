use tokio::io::Join;
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep};

async fn fetch_research_data(id: u8) -> String {
    let delay = Duration::from_secs(2);
    println!("Task {} started: Fetching data...", id);
    sleep(delay).await;
    println!("Task {} finished: Data retrieved.", id);
    format!("Result from Task: {}", id)
}

#[tokio::main]
async fn main() {
    println!("A-ReC Agent Starting Concurrent Fetch...");

    // 1. create a vector to store joinHandles
    let mut handles: Vec<JoinHandle<String>> = Vec::new();

    // 2. spawn 3 tasks concurrently
    for i in 1..=3 {
        let handle = tokio::spawn(fetch_research_data(i));
        handles.push(handle);
    }

    // 3. wait for all spawned tasks to complete sequentially
    println!("\nMain thread is now awaiting the results of the concurrent tasks...");

    let mut results: Vec<String> = Vec::new();
    for handle in handles {
        let result = handle.await.unwrap();
        results.push(result);
    }

    println!("\n✅ All concurrent tasks completed.");
    println!("Final Results: {:?}", results);
    println!("A-ReC Agent Shutting Down.");
}
