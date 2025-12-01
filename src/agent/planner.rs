use std::time::Duration;

use crate::agent::context::AgentContext;
use crate::tools::scrapper::ToolOutput;
use anyhow::Result;
use serde::{Deserialize, Serialize};

// New agents structures
// 1. define the possible actions agent can take.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "arguments")] // (to structure this into type: {search or finish}, arguments:{query or final_answer})
pub enum AgentAction {
    // action to search the web for a specific query
    Search { query: String },
    // action to signal that the goal is complete
    Finish { final_answer: String },
}
// 2. define LLMs full plan response structure
pub struct ActionPlan {
    pub reasoning: String,   // llms chain of though (CoT)
    pub action: AgentAction, // structured action to take next
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
// to accept context, observation
pub async fn reasoning_engine(
    context: &AgentContext,
    observation: ToolOutput,
) -> Result<ActionPlan> {
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
