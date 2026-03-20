use crate::core::state::Dream;
use crate::dream::templates::FRAGMENTS;
use rand::seq::IndexedRandom;
use rand::RngExt;

pub struct Generator;

impl Generator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, id: usize) -> Dream {
        let mut rng = rand::rng();
        let num_lines = rng.random_range(5..15);
        
        let mut lines = Vec::new();
        for _ in 0..num_lines {
            if let Some(&line) = FRAGMENTS.choose(&mut rng) {
                lines.push(line.to_string());
            }
        }

        // Add a random function definition sometimes
        if rng.random_bool(0.3) {
            lines.insert(0, "fn rand_op(a, b) { return (a * b) % (a + b + 1); }".to_string());
            lines.push("print(\"Random operation result: \" + rand_op(x, y));".to_string());
        }

        Dream {
            id,
            code: lines.join("\n"),
            mutation_parent: None,
        }
    }
}
