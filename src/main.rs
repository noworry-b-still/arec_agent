use std::fmt::format;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, sleep};

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

// New agents structures
// 1. define the possible actions agent can take.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "arguments")] // (to structure this into type: {search or finish}, arguments:{query or final_answer})
enum AgentAction {
    // action to search the web for a specific query
    Search { query: String },
    // action to signal that the goal is complete
    Finish { final_answer: String },
}
// 2. define LLMs full plan response structure
struct ActionPlan {
    reasoning: String,   // llms chain of though (CoT)
    action: AgentAction, // structured action to take next
}

// Add a helper method to the AgentAction enum for easy ID extraction in the summary
impl AgentAction {
    fn get_id(&self) -> String {
        match self {
            AgentAction::Search { query } => {
                query.split(": ").last().unwrap_or("Unknown").to_string()
            }
            AgentAction::Finish { final_answer } => final_answer
                .split(" is complete.")
                .next()
                .unwrap_or("Unknown")
                .replace("The status is confirmed: '", ""),
        }
    }
}

// 2. implement mock reasoning function
async fn reasoning_engine(observation: ResearchItem) -> Result<ActionPlan> {
    println!("\n🧠 Reasoning Engine: Analyzing observation...");
    sleep(Duration::from_millis(500)).await; // simulate API latency

    let plan = if observation.is_completed {
        ActionPlan {
            reasoning: format!(
                "The research item ID {} has the title '{}' and is marked completed. No further action needed.",
                observation.id, observation.title
            ),
            action: AgentAction::Finish {
                final_answer: format!(
                    "The status is confimed: '{}' is complete",
                    observation.title
                ),
            },
        }
    } else {
        ActionPlan {
            reasoning: format!(
                "The research item ID {} is incomplete. I need to search for more information on the topic: '{}'.",
                observation.id, observation.title
            ),
            action: AgentAction::Search {
                query: format!("How to complete task: {}", observation.title),
            },
        }
    };

    println!(
        "-> Plan Generated. Action Type: {}",
        match &plan.action {
            AgentAction::Search { .. } => "Search",
            AgentAction::Finish { .. } => "Finish",
        }
    );
    Ok(plan)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n🚀 A-ReC Agent Day 3: Reasoning Engine Test...\n");

    // --- 1. OBSERVE (Fetch two items: one complete (4) and one incomplete (5)) ---
    let handle_4 = tokio::spawn(fetch_and_parse_data(4));
    let handle_5 = tokio::spawn(fetch_and_parse_data(5));

    let obs_4 = handle_4.await??;
    let obs_5 = handle_5.await??;

    // --- 2. REASON/PLAN (Pass observations to the Decision-maker) ---
    let plan_handle_4 = tokio::spawn(reasoning_engine(obs_4));
    let plan_handel_5 = tokio::spawn(reasoning_engine(obs_5));

    let plan_4 = plan_handle_4.await??;
    let plan_5 = plan_handel_5.await??;

    // --- 3. EXECUTE (Simulated logging of the planned Action) ---

    println!("\n==============================================");
    println!("🔬 **A-ReC RESEARCH CYCLE SUMMARY**");
    println!("==============================================");

    // Summary for Item 4 (Completed)
    println!("Item ID {}:", plan_4.action.get_id());
    println!("  Reasoning: {}", plan_4.reasoning);
    println!("  Action: {:?}", plan_4.action);
    println!("----------------------------------------------");

    // Summary for Item 5 (Incomplete)
    println!("Item ID {}:", plan_5.action.get_id());
    println!("  Reasoning: {}", plan_5.reasoning);
    println!("  Action: {:?}", plan_5.action);
    println!("==============================================");

    Ok(())
}
