use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct ToolOutput {
    pub content: String,
}

pub async fn scrape_tool(target_url: &str) -> Result<ToolOutput> {
    println!("\n🕸️  Tool Executing: SCRAPING URL: {}", target_url);
    let client = Client::new();

    let response = client.get(target_url).send().await?;

    let text = response.error_for_status()?.text().await?;

    let document = Html::parse_document(&text);

    let selector = Selector::parse("p").map_err(|_| anyhow::anyhow!("Invalid CSS selector"))?;

    let mut extracted_text = String::new();
    let mut total_length = 0;

    for element in document.select(&selector) {
        let text_content = element.text().collect::<String>().trim().to_string();
        if !text_content.is_empty() {
            extracted_text.push_str(&text_content);
            extracted_text.push(' '); // Add space between paragraphs
            total_length += text_content.len() + 1;
        }
    }
    let snippet_length = total_length.min(500); // Take up to 500 characters
    let final_content = format!(
        "SCRAPED CONTENT (Snippet): {}... (Total chars: {})",
        &extracted_text[..snippet_length],
        total_length
    );

    println!(
        "✅ Tool Finished: Extracted {} characters of text.",
        total_length
    );

    Ok(ToolOutput {
        content: final_content,
    })
}
