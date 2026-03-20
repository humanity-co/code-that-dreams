use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dream {
    pub id: usize,
    pub code: String,
    pub mutation_parent: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u128,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interpretation {
    pub summary: String,
    pub hallucinations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamCycle {
    pub dream: Dream,
    pub result: ExecutionResult,
    pub interpretation: Interpretation,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub history: VecDeque<DreamCycle>,
    pub next_id: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(100),
            next_id: 1,
        }
    }

    pub fn add_cycle(&mut self, cycle: DreamCycle) {
        if self.history.len() >= 100 {
            self.history.pop_front();
        }
        self.history.push_back(cycle);
        self.next_id += 1;
    }
}
