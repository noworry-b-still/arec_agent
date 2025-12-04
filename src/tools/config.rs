use anyhow::{Result, anyhow};
use reqwest::Client;
use std::env;

pub struct SearchClient {
    pub client: Client,
    pub api_key: String,
    pub search_endpoint: String,
}

impl SearchClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("SEARCH_API_KEY")
            .map_err(|_| anyhow!("SEARCH_API_KEY not found. Set it in your .env fike"))?;

        // in real scenario later, this would be actual API Url
        let search_endpoint = env::var("SEARCH_ENDPOINT")
            .unwrap_or_else(|_| "https://api.mocksearch.com/v1/search".to_string());

        // create single reqwest::Client instance for efficiency
        let client = Client::new();
        Ok(Self {
            client,
            api_key,
            search_endpoint,
        })
    }
}

// A struct to hold the LLM client configuration
pub struct LLMClient {
    pub client: Client,
    pub api_key: String,
    pub model: String,
    pub endpoint: String,
}

impl LLMClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("LLM_API_KEY")
            .map_err(|_| anyhow!("LLM_API_KEY not found. Set it in your .env file."))?;

        let model = env::var("LLM_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());

        let endpoint = env::var("LLM_ENDPOINT")
            .unwrap_or_else(|_| "https://api.mockllm.com/v1/chat/completions".to_string());

        let client = Client::new();

        Ok(Self {
            client,
            api_key,
            model,
            endpoint,
        })
    }
}
