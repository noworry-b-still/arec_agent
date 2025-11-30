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
#[derive(Debug)]
struct ToolOutput {
    content: String,
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

// 3. implement the action dispatcher - Execution phase
async fn search_tool(query: &str) -> ToolOutput {
    println!("\n🔍 Tool Executing: Searching for: '{}'", query);
    tokio::time::sleep(Duration::from_secs(1)).await; // Simulate network latency

    let mock_result = format!(
        "Search Result for '{}': The top result suggests using the 'reqwest::Client' to manage connections and handle futures with 'tokio::join!'. Total 3 relevant snippets found.",
        query
    );

    println!("✅ Tool Finished: Received search result.");
    ToolOutput {
        content: mock_result,
    }
}

async fn tool_executor(plan: ActionPlan) -> Result<Option<ToolOutput>> {
    match plan.action {
        AgentAction::Finish { final_answer } => {
            println!("\n🛑 AGENT HALT: Goal Achieved!");
            println!("   Final Answer: {}", final_answer);
            // Return None to signal the main loop to stop
            Ok(None)
        }
        AgentAction::Search { query } => {
            let tool_output = search_tool(&query).await;
            // Return the output wrapped in Some() because the cycle should continue
            Ok(Some(tool_output))
        }
    }
}

//  driver code  - integration of Observer-reason-execute loop
#[tokio::main]
async fn main() -> Result<()> {
    println!("\n🚀 A-ReC Agent Day 4: Full O-R-A Loop (Single Cycle) Test...\n");

    // -- 1. OBSERVE (fetch incomplete todo item 5) --
    let initial_observation = tokio::spawn(fetch_and_parse_data(5)).await??;

    // -- 2. RESON/PLAn( Get the action plan) --
    let plan = tokio::spawn(reasoning_engine(initial_observation)).await??;

    // -- 3. ACT (execute the plan) --
    // The reasoning engine for item 5 should have returned AgentAction::Search since it is incomplete
    println!("\n--- STARTING EXECUTION ---");
    let next_observation = tool_executor(plan).await?;

    // -- 4. PREPARE for the next cycle (Logging the new observeation for now) --
    if let Some(output) = next_observation {
        println!("\n----------------------------------------------");
        println!("✨ Cycle 1 Complete. New Observation Ready for LLM:");
        println!("{}", output.content);
        println!("----------------------------------------------");

        // In a real agent, you would now pass `output.content`
        // back to the `reasoning_engine` for the next step.
    } else {
        println!("\nAgent finished in the first cycle.");
    }
    Ok(())
}
