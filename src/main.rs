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

// -- New Agent Memory Structure --
// Represents an entry in the agent's history (a completed step I mean)
#[derive(Debug, Clone)]
struct HistoryEntry {
    action: String,
    observation: String,
}

#[derive(Debug)]
struct AgentContext {
    goal: String,
    history: Vec<HistoryEntry>,
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
// to accept context, observation
async fn reasoning_engine(context: &AgentContext, observation: ToolOutput) -> Result<ActionPlan> {
    println!("\n🧠 Reasoning Engine: Analyzing observation...");

    // 1.  Simulate using the full history and new observation to make a decision
    let context_summary = format!(
        "Goal: {}. History has {} steps. New Observation: {}",
        context.goal,
        context.history.len(),
        observation.content
    );

    // 2. Simulate complex decision logic based on the observation content
    let plan = if observation.content.contains("The final answer is 42") {
        // Condition1: tool has found the key peice of info
        ActionPlan {
            reasoning: "The previous search has yielded the final, authoritative answer. I can now conclude the research.".to_string(),
            action: AgentAction::Finish {
            final_answer: "The meaning of life, according to the research, is 42.".to_string(),
            },
        }
    } else {
        let new_query = if context.history.is_empty() {
            "initial research query".to_string()
        } else {
            "deeper dive into the number 42's context".to_string()
        };

        ActionPlan {
            reasoning: format!(
                "Initial search provided a general result. I need to run a new targeted search: {}",
                new_query
            ),
            action: AgentAction::Search { query: new_query },
        }
    };

    // 3. (Real LLM step): Simulate API latency
    tokio::time::sleep(Duration::from_millis(500)).await;

    Ok(plan)
}

// 3. implement the action dispatcher - Execution phase
async fn search_tool(query: &str) -> ToolOutput {
    println!("\n🔍 Tool Executing: Searching for: '{}'", query);
    tokio::time::sleep(Duration::from_secs(1)).await; // Simulate network latency

    let mock_result = if query.contains("deeper dive") {
        // Second search yields the final answer
        "Search Result: Deep analysis complete. The final answer is 42. No further research is required.".to_string()
    } else {
        // Initial search yields a hint
        format!(
            "Search Result for '{}': The general finding points toward a key number. I must refine the search.",
            query
        )
    };

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
    println!("\n🚀 A-ReC Agent Day 5: Full Autonomous O-R-A Loop...\n");

    // users intial prompt is 1st observation.
    let initial_goal = "Determin the try meaning of life and the universe.";
    let mut current_observation = ToolOutput {
        content: format!("The user wants to: {}", initial_goal),
    };

    // agent context
    let mut context = AgentContext {
        goal: initial_goal.to_string(),
        history: Vec::new(),
    };

    let mut cycle_count = 0;
    loop {
        cycle_count += 1;
        println!("\n==============================================");
        println!(
            "  CYCLE {} - Agent has {} history steps.",
            cycle_count,
            context.history.len()
        );
        println!("==============================================");

        // --- 1. REASON/PLAN (The Reasoning Engine decides the next action) ---
        let plan = reasoning_engine(&context, current_observation).await?;

        // --- 2. ACT (Execute the plan) ---
        let action_str = format!("{:?}", plan.action);
        let execution_result = tool_executor(plan).await?;

        // --- 3. OBSERVE (Process the execution result for the next cycle) ---
        match execution_result {
            // Case 1: The agent executed a tool (e.g., Search). The cycle continues.
            Some(next_tool_output) => {
                // Record the completed step in the agent's memory
                context.history.push(HistoryEntry {
                    action: action_str,
                    observation: next_tool_output.content.clone(),
                });

                // Set the output as the observation for the next loop iteration
                current_observation = next_tool_output;
            }
            // Case 2: The agent returned the Finish action. The loop must break.
            None => {
                break;
            }
        }
    }

    println!(
        "\n✅ Autonomous Research Complete in {} cycles.",
        cycle_count
    );
    println!("Agent finished.");
    Ok(())
}
