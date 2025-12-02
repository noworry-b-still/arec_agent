// src/main.rs

// Declare the modules created during refactoring
mod agent;
mod tools;

// --- Imports for Orchestration and Execution ---
use anyhow::Result;
use tokio; // Used implicitly by #[tokio::main]

// Imports from Agent Logic (Memory and Planning)
use agent::context::{AgentContext, HistoryEntry};
use agent::planner::AgentAction;
use agent::planner::reasoning_engine; // To format the action string

// Imports from Tools (The Observation Structure and Execution Dispatcher)
use tools::ToolOutput;
use tools::executor::tool_executor;

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n🚀 A-ReC Agent Day 6: Modular Autonomous Loop Running...\n");

    // --- 1. INITIALIZE AGENT CONTEXT (Memory) ---
    let initial_goal =
        "Determine the content of the very first webpage ever created and find its source.";
    let mut context = AgentContext {
        goal: initial_goal.to_string(),
        history: Vec::new(),
    };

    // The very first observation is the user's initial prompt (the goal)
    let mut current_observation = ToolOutput {
        content: format!("The user wants to find information on: {}", initial_goal),
    };

    let mut cycle_count = 0;

    // --- 2. THE AUTONOMOUS O-R-A LOOP ---
    loop {
        cycle_count += 1;

        // Safety break to prevent infinite loops (common in agent development)
        if cycle_count > 5 {
            eprintln!("\n🛑 Safety break: Agent exceeded 5 cycles. Halting.");
            break;
        }

        println!("\n==============================================");
        println!(
            "  CYCLE {} - Agent has {} history steps.",
            cycle_count,
            context.history.len()
        );
        println!("==============================================");

        // --- REASON/PLAN (The Reasoning Engine decides the next action) ---
        // Pass the context and the latest observation to the planner
        let plan = reasoning_engine(&context, &current_observation).await?;

        // --- ACT (Execute the plan) ---
        let action_str = format!("{:?}", plan.action);
        println!("📝 Planned Action: {:?}", plan.action);

        let execution_result = tool_executor(plan).await?;

        // --- OBSERVE (Process the execution result for the next cycle) ---
        match execution_result {
            // Case 1: The agent executed a tool (e.g., Search or Scrape). The cycle continues.
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
    println!("Final Context History: {:#?}", context.history);

    Ok(())
}
