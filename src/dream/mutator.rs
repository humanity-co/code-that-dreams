use crate::core::state::Dream;
use rand::seq::IndexedRandom;
use rand::RngExt;

pub struct Mutator;

impl Mutator {
    pub fn new() -> Self {
        Self
    }

    pub fn mutate(&self, dream: &Dream, new_id: usize) -> Dream {
        let mut rng = rand::rng();
        let mut lines: Vec<String> = dream.code.lines().map(|s| s.to_string()).collect();
        
        let mutation_type = rng.random_range(0..3);
        
        match mutation_type {
            0 => {
                // Swap two lines
                if lines.len() >= 2 {
                    let i = rng.random_range(0..lines.len());
                    let j = rng.random_range(0..lines.len());
                    lines.swap(i, j);
                }
            }
            1 => {
                // Remove a random line
                if lines.len() > 5 {
                    let i = rng.random_range(0..lines.len());
                    lines.remove(i);
                }
            }
            2 => {
                // Invert a condition if found
                for line in lines.iter_mut() {
                    if line.contains(">") {
                        *line = line.replace(">", "<");
                        break;
                    } else if line.contains("<") {
                        *line = line.replace("<", ">");
                        break;
                    }
                }
            }
            _ => {}
        }

        Dream {
            id: new_id,
            code: lines.join("\n"),
            mutation_parent: Some(dream.id),
        }
    }
}
