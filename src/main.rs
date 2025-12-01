mod agent;
mod tools;


use anyhow::Result;
use tokio;
use agent::context::AgentContext;
use tools::executor::tool_executor;
use tools::ToolOutput;
use serde::{Deserialize};


// we have simple todo item as research item
#[derive(Debug, Deserialize)]
struct ResearchItem {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,
    title: String,
    #[serde(rename = "completed")]
    is_completed: bool,
}

// 1. implement observation function
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

//  driver code  - integration of Observer-reason-execute loop
#[tokio::main]
async fn main() -> Result<()> {
    println!("\n🚀 A-ReC Agent Day 5: Full Autonomous O-R-A Loop...\n");
    Ok(())
}
