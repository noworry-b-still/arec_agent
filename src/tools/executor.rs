use crate::agent::planner::{ActionPlan, AgentAction};
use crate::tools::research::search_tool;
use crate::tools::scrapper::ToolOutput;
use anyhow::Result;

pub async fn tool_executor(plan: ActionPlan) -> Result<Option<ToolOutput>> {
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
