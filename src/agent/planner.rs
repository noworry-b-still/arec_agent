use crate::agent::context::AgentContext;
use crate::tools::ToolOutput;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, sleep};

// The AgentAction enum (ensure it is public and derives Clone)
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "arguments")]
pub enum AgentAction {
    Search { query: String },
    Scrape { url: String },
    Finish { final_answer: String },
}

// The ActionPlan struct
#[derive(Debug, Serialize, Deserialize)]
pub struct ActionPlan {
    pub reasoning: String,
    pub action: AgentAction,
}

// The Mock Reasoning Engine
pub async fn reasoning_engine(
    context: &AgentContext,
    observation: &ToolOutput,
) -> Result<ActionPlan> {
    println!(
        "\n🧠 Reasoning Engine: Analyzing observation (History: {} steps)...",
        context.history.len()
    );
    sleep(Duration::from_millis(500)).await;

    // Logic based on the current state/observation

    if observation.content.contains("all the countries") {
        // SCENARIO 3: Scraped content confirms the answer.
        return Ok(ActionPlan {
                reasoning: "The scrape provided a list of countries, which confirms the agent's ability to extract content. We can now conclude this test.".to_string(),
                action: AgentAction::Finish {
                    final_answer: "The research successfully scraped a list of countries from the target page, demonstrating tool competency.".to_string(),
                },
        });
    } else if observation.content.contains("Found URL: http") {
        // SCENARIO 2: Search provided a URL. Extract and scrape it.
        let url_to_scrape = observation
            .content
            .split("Found URL: ")
            .nth(1)
            .unwrap()
            .to_string();
        return Ok(ActionPlan {
            reasoning: format!(
                "Initial search returned a relevant article URL ({}). I must now scrape the page for detailed content.",
                url_to_scrape
            ),
            action: AgentAction::Scrape { url: url_to_scrape },
        });
    } else {
        // SCENARIO 1: No specific URL found, or initial state. Search for the goal.
        let search_query = "What was the content of the very first webpage?";
        return Ok(ActionPlan {
            reasoning: format!(
                "Starting research for goal: {}. Beginning with a targeted search for the first webpage content.",
                context.goal
            ),
            action: AgentAction::Search {
                query: search_query.to_string(),
            },
        });
    }
}
