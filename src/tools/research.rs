use super::scraper::ToolOutput; // Import ToolOutput from the same module group
use std::time::Duration;
use tokio::time::sleep;

// Mock tool: Simulates a search API call that returns a URL.
pub async fn search_tool(query: &str) -> ToolOutput {
    println!("\n🔍 Tool Executing: Searching for: '{}'", query);
    sleep(Duration::from_millis(500)).await;

    // We use the URL of the first website ever (info.cern.ch) for a reliable test.
    let mock_url = "https://www.scrapethissite.com/pages/";

    let mock_result = format!(
        "Search Result for '{}': Top article found. Found URL: {}",
        query, mock_url
    );

    println!("✅ Tool Finished: Found relevant URL.");
    ToolOutput {
        content: mock_result,
    }
}
