// -- New Agent Memory Structure --
// Represents an entry in the agent's history (a completed step I mean)
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub action: String,
    pub observation: String,
}

#[derive(Debug)]
pub struct AgentContext {
    pub goal: String,
    pub history: Vec<HistoryEntry>,
}
