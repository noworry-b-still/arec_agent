use super::config::LLMClient;
use crate::agent::context::AgentContext;
use crate::agent::planner::ActionPlan; // Removed AgentAction
use crate::tools::ToolOutput;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize}; // Added Serialize
use serde_json::json;

// --- LLM API Request/Response Structures (Simplified) ---

// Represents a single message role (system, user, assistant)
#[derive(Serialize, Deserialize)] // Added Serialize
struct Message {
    role: String,
    content: String,
}

// Represents the payload sent to the LLM API
#[derive(Serialize)] // Changed from Deserialize to Serialize
struct LLMRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<serde_json::Value>,
}

// Represents the critical part of the LLM API response structure
#[derive(Deserialize)] // Removed serde::
struct LLMResponseChoice {
    message: Message,
}

#[derive(Deserialize)] // Removed serde::
struct LLMResponse {
    choices: Vec<LLMResponseChoice>,
}

// The core function that constructs the prompt and calls the LLM
pub async fn call_llm_for_plan(
    context: &AgentContext,
    observation: &ToolOutput,
) -> Result<ActionPlan> {
    // 1. Initialize Client
    let client = LLMClient::new()?;

    // 2. Define the System Prompt (Crucial for Agentic AI)
    // We dynamically generate the required JSON schema from the Rust structs
    let json_schema = json!({
        "type": "object",
        "properties": {
            "reasoning": {"type": "string", "description": "Your detailed thought process (Chain-of-Thought) for the next action."},
            "action": {
                "type": "object",
                "oneOf": [
                    {"type": "object", "properties": {"type": {"const": "Search"}, "arguments": {"type": "object", "properties": {"query": {"type": "string"}}}}},
                    {"type": "object", "properties": {"type": {"const": "Scrape"}, "arguments": {"type": "object", "properties": {"url": {"type": "string"}}}}},
                    {"type": "object", "properties": {"type": {"const": "Finish"}, "arguments": {"type": "object", "properties": {"final_answer": {"type": "string"}}}}}
                ]
            }
        },
        "required": ["reasoning", "action"]
    });

    let system_prompt = format!(
        "You are an Autonomous Research Code (A-ReC) Agent. Your goal is: {}.
        You MUST follow the Observe-Reason-Act loop.
        You have used tools and received observations.
        Your available tools are: Search (query), Scrape (url).

        Your RESPONSE MUST BE a single, valid JSON object that adheres to the following JSON schema. DO NOT include any text outside the JSON block:
        {}",
        context.goal,
        json_schema.to_string()
    );

    let mut messages = vec![Message {
        role: "system".to_string(),
        content: system_prompt,
    }];

    // 3. Add History (Previous O-R-A steps)
    for entry in &context.history {
        // Assistant's previous action (simulated LLM output)
        messages.push(Message {
            role: "assistant".to_string(),
            content: format!("{{\"reasoning\": \"...\", \"action\": {}}}", entry.action), // Simplified action representation
        });
        // User's observation (tool output)
        messages.push(Message {
            role: "user".to_string(),
            content: format!("Observation: {}", entry.observation),
        });
    }

    // 4. Add Current Observation
    messages.push(Message {
        role: "user".to_string(),
        content: format!("NEW OBSERVATION: {}", observation.content),
    });

    // 5. Construct Request
    let request = LLMRequest {
        model: client.model.clone(),
        messages,
        response_format: Some(json!({ "type": "json_object" })), // Crucial for structured output
    };

    // --- MOCK API CALL START (Replace with real client.post().send().await? later) ---
    // Simulate the API returning a perfect JSON plan based on the observation:
    let inner_content: String;

    if observation.content.contains("self-governing") {
        // Success case (The scraped content has the definition)
        inner_content = r#"{"reasoning":"The scrape successfully retrieved the definition of autonomy, explicitly mentioning 'self-governing'. The research goal is now complete.","action":{"type":"Finish","arguments":{"final_answer":"Autonomy is defined as the capacity for informed, uncoerced decision-making, and often refers to institutions that are self-governing."}}}"#.to_string();
    } else if observation.content.contains("Found URL") {
        // --- FIX: Extract ONLY the clean URL ---
        // 1. Find the token "Found URL: "
        let token = "Found URL: ";
        let url_start_index = observation
            .content
            .find(token)
            .map(|i| i + token.len())
            .unwrap_or(0);

        // 2. Find the end of the URL, which is the next newline character (\n)
        let url_end_index = observation.content[url_start_index..]
            .find('\n')
            .map(|i| i + url_start_index)
            .unwrap_or(observation.content.len());

        // 3. Extract the clean URL string
        let url = observation.content[url_start_index..url_end_index]
            .trim()
            .to_string();

        // Use json! macro to safely construct the inner JSON object
        let plan_json = json!({
            "reasoning": format!("Initial search returned a relevant article URL ({}). I must now scrape the page for detailed content.", url),
            "action": {
                "type": "Scrape",
                "arguments": {
                    "url": url
                }
            }
        });
        // Serialize the JSON object into a string for the inner 'content'
        inner_content = plan_json.to_string();
    } else {
        // Initial search case (from Day 6)
        inner_content = r#"{"reasoning":"Starting research for the goal. I need to perform an initial search to find relevant URLs and snippets.","action":{"type":"Search","arguments":{"query":"Define autonomy and self-governance in AI systems."}}}"#.to_string();
    };

    // 6. Construct the full mock API response using the inner_content string
    let mock_json_response_text = format!(
        r#"{{"choices":[{{"message":{{"role":"assistant","content":{}}}}}]{}}}"#,
        serde_json::to_string(&inner_content)?, // <-- CRITICAL: Use serde_json::to_string to correctly escape the inner string
        ""
    );

    let mock_api_response: LLMResponse = serde_json::from_str(&mock_json_response_text)
        .map_err(|e| anyhow!("Failed to parse mock LLM response: {}", e))?;
    // --- MOCK API CALL END ---

    // 6. Extract and Parse the Final Plan
    let choice = mock_api_response
        .choices
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("LLM returned no response choice."))?;

    // The LLM returns a string inside its message content. We parse that string (the JSON plan).
    let plan_content = choice.message.content;

    // IMPORTANT: The LLM should return a JSON string, which we deserialize into ActionPlan.
    let full_plan: ActionPlan = serde_json::from_str(&plan_content)
        .map_err(|e| anyhow!("Failed to deserialize structured JSON plan from LLM: {}", e))?;

    Ok(full_plan)
}
