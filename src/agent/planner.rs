use crate::agent::context::AgentContext;
use crate::tools::ToolOutput;
use crate::tools::llm::call_llm_for_plan; // NEW IMPORT
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, sleep};

// The AgentAction enum (unchanged)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "arguments")]
pub enum AgentAction {
    Search { query: String },
    Scrape { url: String },
    Finish { final_answer: String },
}

// The ActionPlan struct (unchanged)
#[derive(Debug, Serialize, Deserialize)]
pub struct ActionPlan {
    pub reasoning: String,
    pub action: AgentAction,
}

// The REAL Reasoning Engine Wrapper
pub async fn reasoning_engine(
    context: &AgentContext,
    observation: &ToolOutput,
) -> Result<ActionPlan> {
    // This is now just a clean call to the LLM interaction module
    let plan = call_llm_for_plan(context, observation).await?;

    // Print the reasoning *after* receiving it from the LLM
    println!(
        "\n🧠 Reasoning Engine: Plan received. LLM Reasoning: {}",
        plan.reasoning
    );

    Ok(plan)
}
