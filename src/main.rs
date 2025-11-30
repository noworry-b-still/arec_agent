use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ResearchItem {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,
    title: String,
    #[serde(rename = "completed")]
    is_completed: bool,
}

use anyhow::Result;

async fn fetch_and_parse_data(item_id: u32) -> Result<ResearchItem> {
    println!("-> Task {}: Initiating request.", item_id);

    let url = format!("https://jsonplaceholder.typicode.com/todos/{}", item_id);

    // 1. Make a Get request
    let response = reqwest::get(&url).await?;
    
    // 2 use .json()
    let item = response.json::<ResearchItem>().await?;
    println!("<- Task {}: Data received and parsed.", item_id);

    Ok(item)
}
#[tokio::main]
async fn main() -> Result<()>{
    println!("\n🚀 A-ReC Agent Day 2: Real I/O Observation Demo...\n");

    let mut handles = Vec::new();

    for i in 1..=3 {
        let handle = tokio::spawn(fetch_and_parse_data(i));
        handles.push(handle);
    }
    println!("Main thread awaiting results...\n");

    for handle in handles {
        match handle.await? {
            Ok(item) => println!(
                "[SUCCESS] Item {}: Title='{}', Completed={}",
                item.id, item.title, item.is_completed
            ),
            Err(e) => eprintln!("[ERROR] Failed to fetch item: {}", e),
        }
    }
    println!("\n✅ All concurrent tasks processed.");
    Ok(())
}
