// src/tools/research.rs

use super::config::SearchClient;
use super::scraper::ToolOutput;
use anyhow::{Result, anyhow};
use serde::Deserialize;
use serde_json;
use tokio::time::{Duration, sleep}; // Needed for deserializing the mock JSON

// Add a mutable counter (or rely on the query string for testing reflection)
use std::sync::atomic::{AtomicUsize, Ordering};
static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

// --- New Structures for Search API JSON ---

// 1. Represents a single search result item
#[derive(Debug, Deserialize)]
pub struct SearchItem {
    pub title: String,
    pub snippet: String,
    pub link: String,
}

// 2. Represents the entire search response
#[derive(Debug, Deserialize)]
struct SearchResponse {
    pub items: Option<Vec<SearchItem>>,
}

// The REAL Search Tool Implementation
pub async fn search_tool(query: &str) -> Result<ToolOutput> {
    println!("\n🔍 Tool Executing: Real Search for: '{}'", query);

    let count = CALL_COUNT.fetch_add(1, Ordering::SeqCst) + 1;

    // --- MOCK FAILURE SCENARIO ---
    if count >= 1 {
        println!("🚨 MOCK FAILURE: Simulating API Key expiration or quota limit.");
        // Return a structured error using anyhow
        return Err(anyhow::anyhow!(
            "API Key invalid or quota exceeded on call {}",
            count
        ));
    }

    // 1. Load the client and configuration
    let search_client = SearchClient::new()?;
    let url = format!(
        "{}?q={}&key={}",
        search_client.search_endpoint, query, search_client.api_key
    );

    // 2. MOCK API CALL (Simulating response and parsing)

    // Simulate API call latency
    sleep(Duration::from_millis(800)).await;

    // This JSON mimics what a real search API returns
    let mock_json_response = r#"{
        "items": [
            {
                "title": "Wikipedia: Autonomy (self-governance)",
                "snippet": "The concept of self-governance or self-rule, a key component of agentic systems.",
                "link": "https://en.wikipedia.org/wiki/Autonomy"
            },
            {
                "title": "History of AI Agents",
                "snippet": "A general overview of the history and architecture of reactive vs. deliberative agents.",
                "link": "https://example.com/ai-history"
            }
        ]
    }"#;

    // 3. Deserialize the mock JSON into our structured response
    let response: SearchResponse = serde_json::from_str(mock_json_response)
        .map_err(|e| anyhow!("Failed to parse Search API JSON: {}", e))?;

    // 4. Process the results and format the new observation for the LLM
    let mut observation_content = format!("Search results for query: '{}'\n", query);

    if let Some(items) = &response.items {
        for (i, item) in items.iter().enumerate() {
            // Check if this result contains the desired URL for the scraper (Wikipedia Autonomy page)
            if item.link.contains("wikipedia.org/wiki/Autonomy") {
                // This is the CRITICAL line that enables the next step (Scrape)
                observation_content.push_str(&format!("Found URL: {}\n", item.link));
            }
            observation_content.push_str(&format!(
                "Result {}: Title: {}, Snippet: {}\n",
                i + 1,
                item.title,
                item.snippet
            ));
        }
    } else {
        observation_content.push_str("No relevant search results found.");
    }

    println!(
        "✅ Tool Finished: Parsed {} results.",
        response.items.as_ref().map_or(0, |i| i.len())
    );

    // Make sure we return a Result<ToolOutput>
    Ok(ToolOutput {
        content: observation_content,
    })
}
