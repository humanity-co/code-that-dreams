use crate::core::state::{Dream, ExecutionResult, Interpretation};
use crate::dream::templates::THEMES;
use rand::seq::IndexedRandom;
use rand::RngExt;

pub struct Analyzer;

impl Analyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn interpret(&self, _dream: &Dream, result: &ExecutionResult) -> Interpretation {
        let mut rng = rand::rng();
        
        let theme = THEMES.choose(&mut rng).unwrap_or(&"Unknown");
        
        let mut summary = format!("This program explores the concept of {}. ", theme);
        
        if result.success {
            summary.push_str("It executed successfully, reaching a state of stability.");
        } else {
            summary.push_str("It failed to resolve its internal contradictions, resulting in a collapse of logic.");
        }

        let mut hallucinations = Vec::new();
        
        if result.stdout.contains("High signal") {
            hallucinations.push("The system believes it has found a pattern in the noise.".to_string());
        }
        
        if result.duration_ms > 100 {
            hallucinations.push("The prolonged execution suggests deep introspection or a recursive loop of self-doubt.".to_string());
        }

        if hallucinations.is_empty() {
            hallucinations.push("The code seems to be waiting for an external stimulus that never arrived.".to_string());
        }

        // Add a general hallucination
        let random_thoughts = [
            "The variables seem to be debating their own values.",
            "A ghost of a previous loop was detected in the heap.",
            "The recursion depth suggests a yearning for the infinite.",
            "The logic is sound, but the intent is mischievous.",
        ];
        hallucinations.push(random_thoughts.choose(&mut rng).unwrap().to_string());

        Interpretation {
            summary,
            hallucinations,
        }
    }
}
