use super::research::search_tool;
use super::scraper::{ToolOutput, scrape_tool};
use crate::agent::planner::ActionPlan;
use crate::agent::planner::AgentAction;
use anyhow::Result;

// The Execution Engine/Dispatcher
pub async fn tool_executor(plan: ActionPlan) -> Option<ToolOutput> {
    let result: Result<Option<ToolOutput>> = match plan.action {
        AgentAction::Search { query } => {
            // error is caught here
            let tool_output = search_tool(&query).await;
            Ok(Some(tool_output.unwrap_or_else(|e| ToolOutput {
                content: format!("TOOL_ERROR: Search failed. Reason: {}", e),
            })))
        }

        AgentAction::Scrape { url } => {
            // Call the actual scraping tool
            let tool_output = scrape_tool(&url).await;
            Ok(Some(tool_output.unwrap_or_else(|e| ToolOutput {
                content: format!("TOOL_ERROR: Scrape failed. Reason: {}", e),
            })))
        }

        AgentAction::Finish { final_answer } => {
            println!("\n🛑 AGENT HALT: Goal Achieved!");
            println!("   Final Answer: {}", final_answer);
            Ok(None)
        }
    };
    // Return the Option<ToolOutput> directly, handling any unexpected top-level error
    match result {
        Ok(output_opt) => output_opt,
        Err(e) => {
            // This should ideally not happen if tool-level errors are caught above
            eprintln!("\nCRITICAL EXECUTION ERROR: {}", e);
            Some(ToolOutput {
                content: format!("CRITICAL_ERROR: Execution failed: {}", e),
            })
        }
    }
}

// notes;
// I changed  tool_executor to return Option<ToolOutput> and handle the Result internally,
// converting any error into a ToolOutput whose content starts with
// TOOL_ERROR: This formatted observation is then fed back to the LLM.
