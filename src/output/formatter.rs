use crate::core::state::{Dream, ExecutionResult, Interpretation};

pub struct Formatter;

impl Formatter {
    pub fn new() -> Self {
        Self
    }

    pub fn format_dream_header(&self, dream: &Dream) -> String {
        format!("\n--- 🌀 DREAM #{} ---\n", dream.id)
    }

    pub fn format_execution_result(&self, result: &ExecutionResult) -> String {
        let status = if result.success { "SUCCESS" } else { "FAILURE" };
        let mut output = format!("\n[Execution Output - {} ({}ms)]\n", status, result.duration_ms);
        if !result.stdout.is_empty() {
            output.push_str(&result.stdout);
        }
        if !result.stderr.is_empty() {
            output.push_str("Error: ");
            output.push_str(&result.stderr);
        }
        output
    }

    pub fn format_interpretation(&self, interpretation: &Interpretation) -> String {
        let mut output = format!("\n[Interpretation]\n{}\n", interpretation.summary);
        for hallucination in &interpretation.hallucinations {
            output.push_str(&format!("✨ {}\n", hallucination));
        }
        output
    }
}
