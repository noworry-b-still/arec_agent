use super::research::search_tool;
use super::scraper::{ToolOutput, scrape_tool};
use crate::agent::planner::ActionPlan;
use crate::agent::planner::AgentAction;
use anyhow::Result;

// The Execution Engine/Dispatcher
pub async fn tool_executor(plan: ActionPlan) -> Result<Option<ToolOutput>> {
    match plan.action {
        AgentAction::Search { query } => {
            let tool_output = search_tool(&query).await;
            Ok(Some(tool_output))
        }

        // NEW MATCH ARM
        AgentAction::Scrape { url } => {
            // Call the actual scraping tool
            let tool_output = scrape_tool(&url).await?;
            Ok(Some(tool_output))
        }

        AgentAction::Finish { final_answer } => {
            println!("\n🛑 AGENT HALT: Goal Achieved!");
            println!("   Final Answer: {}", final_answer);
            Ok(None)
        }
    }
}
