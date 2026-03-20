use rhai::{Engine, Scope, OptimizationLevel};
use crate::core::state::ExecutionResult;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self
    }

    fn create_engine(&self) -> Engine {
        let mut engine = Engine::new();
        engine.set_optimization_level(OptimizationLevel::Simple);
        engine.disable_symbol("eval");
        engine
    }

    pub async fn execute(&self, code: &str) -> ExecutionResult {
        let mut scope = Scope::new();
        let start = Instant::now();
        
        // Set up a way to capture output
        let output = Arc::new(Mutex::new(String::new()));
        let output_clone = output.clone();
        
        let mut engine = self.create_engine();
        
        // Override print to capture it
        engine.on_print(move |s| {
            let mut out = output_clone.lock().unwrap();
            out.push_str(s);
            out.push('\n');
        });

        // Set limits
        let max_duration = Duration::from_millis(500);
        
        // We can use a progress callback to check for time limits
        let start_inner = Instant::now();
        engine.on_progress(move |count| {
            if count % 1000 == 0 && start_inner.elapsed() > max_duration {
                Some(rhai::Dynamic::from("Execution timed out".to_string()))
            } else {
                None
            }
        });

        match engine.run_with_scope(&mut scope, code) {
            Ok(_) => {
                let stdout = output.lock().unwrap().clone();
                ExecutionResult {
                    stdout,
                    stderr: String::new(),
                    duration_ms: start.elapsed().as_millis(),
                    success: true,
                }
            }
            Err(e) => {
                let stdout = output.lock().unwrap().clone();
                ExecutionResult {
                    stdout,
                    stderr: e.to_string(),
                    duration_ms: start.elapsed().as_millis(),
                    success: false,
                }
            }
        }
    }
}
