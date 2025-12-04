# 🤖 A-ReC: Autonomous Research Code Agent (21-Day Mastery)

[![Rust](https://img.shields.io/badge/Language-Rust-orange)](https://www.rust-lang.org/)
[![Runtime](https://img.shields.io/badge/Async_Runtime-Tokio-blue)](https://tokio.rs/)
[![Architecture](https://img.shields.io/badge/Architecture-Agentic%20AI%20(O--R--A%20Loop)-red)](https://en.wikipedia.org/wiki/Intelligent_agent)

---

## 💡 Project Goal

This project is a 21-day intensive effort to design and build an **Autonomous Research Code Agent (A-ReC)** from scratch using Rust.

The primary goals are:
1.  **Master Rust and Tokio:** Achieve expert-level proficiency in asynchronous programming for building high-performance I/O applications.
2.  **Agentic AI Development:** Implement a robust and modular architecture based on the **Observe-Reason-Act (O-R-A)** loop, enabling the agent to autonomously research complex queries.
3.  **Ship an Agent:** Deliver a working, high-performance research agent by the end of the 21-day period.

## 🚀 Technology Stack

| Component | Technology | Role |
| :--- | :--- | :--- |
| **Language** | `Rust` | Performance, Safety, and Concurrency. |
| **Runtime** | `Tokio` | Asynchronous, multi-threaded execution for I/O bound tasks (network requests). |
| **Data** | `reqwest`, `serde` | Handling HTTP requests and JSON (LLM/API) data. |
| **Reasoning** | *(Planned)* | Integration with an LLM via API (e.g., OpenAI, Anthropic, or Local LLM). |

## 🏗️ Architecture Principle: The O-R-A Loop

The A-ReC agent is structured around the fundamental loop of Agentic AI:

1.  **Observe (Perception):** Gather data from the environment (e.g., user input, search results, tool outputs).
2.  **Reason/Plan (Cognition):** Use the LLM to analyze the observation, update the internal state (memory), and generate a new action plan.
3.  **Act (Execution):** Execute the plan by calling a specific tool (e.g., Web Search, Scraper, Code Interpreter).

---

### ➡️ Day 1: Laying the Foundation (Tokio Concurrency)

| Concept | Status | Notes |
| :--- | :--- | :--- |
| **Agentic AI Conceptualized** | **Completed** | Defined the Observe-Reason-Act (O-R-A) Loop as the high-level architecture. |
| **Tokio Setup** | **Completed** | Project initialized with `tokio = { features = ["full"] }` for async runtime support. |
| **Concurrency** | **Completed** | Used `tokio::spawn` and `JoinHandle` to run multiple I/O-bound tasks concurrently, demonstrating non-blocking behavior with `sleep`. |

### ➡️ Day 2: The Observe Layer (Real I/O and Data Parsing)

| Concept | Status | Notes |
| :--- | :--- | :--- |
| **Async HTTP** | **Completed** | Integrated `reqwest` for non-blocking network requests. |
| **Data Parsing** | **Completed** | Used `serde` (`Deserialize`) to convert JSON API responses into strongly-typed Rust structs (`ResearchItem`). |
| **Error Handling** | **Completed** | Adopted `anyhow` and the `?` operator for clean, application-level error management across I/O operations. |

### ➡️ Day 3: The Reasoning Engine (LLM Structure and Planning)

| Concept | Status | Notes |
| :--- | :--- | :--- |
| **Structured Output** | **Completed** | Defined the `ActionPlan` struct and `AgentAction` enum to force the LLM to output a predictable JSON schema. |
| **Planning Schema** | **Completed** | Implemented the `Search` and `Finish` actions using `#[serde(tag = "type", ...)]`. |
| **Reasoning Flow** | **Completed** | Created a mock `reasoning_engine` to simulate LLM logic, demonstrating how an observation (input) dictates a specific structured action (output). |

### ➡️ Day 4: The Execution Layer (Act Phase)

| Concept | Status | Notes |
| :--- | :--- | :--- |
| **Execution Interface** | **Completed** | Implemented the `tool_executor` function to dispatch structured `AgentAction`s. |
| **Mock Tooling** | **Completed** | Created a mock `search_tool` to simulate external API calls using `tokio::time::sleep`. |
| **O-R-A Integration** | **Completed** | Integrated all three phases (`Observe`, `Reason`, `Act`) into a single cycle, demonstrating how a plan is executed. |
| **Observation Flow** | **Completed** | Structured the `ToolOutput` as the next Observation, completing the feedback loop for multi-step reasoning. |

### ➡️ Day 5: Persistence and Iteration (The Autonomous Loop)

| Concept | Status | Notes |
| :--- | :--- | :--- |
| **Agent Memory** | **Completed** | Defined the `AgentContext` and `HistoryEntry` structs to manage the agent's state and conversation history. |
| **Iterative Loop** | **Completed** | Wrapped the O-R-A flow in a `loop` structure, allowing the agent to perform multi-step, autonomous reasoning. |
| **Feedback Loop** | **Completed** | The `ToolOutput` from the **Act** phase is now passed as the Observation input to the **Reason** phase of the next cycle. |
| **Loop Termination**| **Completed** | The loop correctly terminates when the `AgentAction::Finish` is returned by the Reasoning Engine. |

### ➡️ Day 6: Advanced Tooling & Modularity (Web Scraping)

| Concept | Status | Notes |
| :--- | :--- | :--- |
| **Project Modularity** | **Completed** | Refactored the monolithic `main.rs` into dedicated `agent/` (brain) and `tools/` (hands) modules for clean structure. |
| **Real Tool Integration** | **Completed** | Added `scraper` and `url` crates to enable real HTML parsing and low-level web interaction. |
| **Scraping Functionality** | **Completed** | Implemented a production-like `scrape_tool` using `reqwest` + CSS selectors to extract readable text from HTML pages. |
| **Multi-Step Planning** | **Completed** | Agent now performs a multi-hop chain: **Search → Scrape → Reason**, enabling richer planning and real web data use. |

### ➡️ Day 7: The Perception Layer (Real Search + Config)

| Concept | Status | Notes |
| :--- | :--- | :--- |
| **Configuration** | **Completed** | Introduced the `dotenvy` crate and a `config.rs` module to safely manage API keys and settings using environment variables. |
| **Stateful Client** | **Completed** | Implemented a `SearchClient` struct to hold and reuse a single `reqwest::Client` instance for optimized, persistent I/O. |
| **Real Search Structure** | **Completed** | Replaced the mock `search_tool` with a real implementation capable of parsing complex JSON API responses (`SearchResponse`) into Rust structs. |
| **Perception Layer Complete** | **Milestone** | The agent's **Observe** layer is fully functional — both Search and Scrape now work with live external data. |

### ➡️ Day 8: LLM Integration (First Real LLM Call)

| Concept | Status | Notes |
| :--- | :--- | :--- |
| **LLM Configuration** | **Completed** | Added `LLMClient` to `config.rs` and updated `.env` to manage LLM API keys and model choice. |
| **Prompt Engineering** | **Completed** | Designed the Prompt Template to include the **System Prompt**, **History**, and **Current Observation**, providing the LLM with full context. |
| **Structured Output** | **Completed** | Defined `LLMRequest` and `LLMResponse` structures and implemented logic to enforce **structured JSON output**, ensuring the LLM's response maps directly to `ActionPlan`. |
| **Reasoning Engine Functional** | **Milestone** | The mock logic has been replaced with a fully structured and callable LLM service wrapper, completing the core cognitive component. |
