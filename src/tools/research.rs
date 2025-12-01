use crate::tools::ToolOutput;
use tokio::time::Duration;

// 3. implement the action dispatcher - Execution phase
pub async fn search_tool(query: &str) -> ToolOutput {
    println!("\n🔍 Tool Executing: Searching for: '{}'", query);
    tokio::time::sleep(Duration::from_secs(1)).await; // Simulate network latency

    let mock_result = if query.contains("deeper dive") {
        // Second search yields the final answer
        "Search Result: Deep analysis complete. The final answer is 42. No further research is required.".to_string()
    } else {
        // Initial search yields a hint
        format!(
            "Search Result for '{}': The general finding points toward a key number. I must refine the search.",
            query
        )
    };

    println!("✅ Tool Finished: Received search result.");
    ToolOutput {
        content: mock_result,
    }
}
