use crate::core::state::State;
use crate::dream::generator::Generator;
use crate::dream::mutator::Mutator;
use crate::sandbox::executor::Executor;
use crate::interpreter::analyzer::Analyzer;
use crate::output::formatter::Formatter;

pub struct Engine {
    state: State,
    generator: Generator,
    mutator: Mutator,
    executor: Executor,
    analyzer: Analyzer,
    formatter: Formatter,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            state: State::new(),
            generator: Generator::new(),
            mutator: Mutator::new(),
            executor: Executor::new(),
            analyzer: Analyzer::new(),
            formatter: Formatter::new(),
        }
    }

    pub async fn run_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let dream = if self.state.history.is_empty() || rand::random::<f64>() < 0.3 {
                self.generator.generate(self.state.next_id)
            } else {
                let parent = self.state.history.back().unwrap();
                self.mutator.mutate(&parent.dream, self.state.next_id)
            };
            
            println!("{}", self.formatter.format_dream_header(&dream));
            if let Some(parent_id) = dream.mutation_parent {
                println!("[Evolved from Dream #{}]", parent_id);
            }
            println!("{}", dream.code);
            
            let result = self.executor.execute(&dream.code).await;
            
            println!("{}", self.formatter.format_execution_result(&result));
            
            let interpretation = self.analyzer.interpret(&dream, &result);
            
            println!("{}", self.formatter.format_interpretation(&interpretation));
            
            self.state.add_cycle(crate::core::state::DreamCycle {
                dream,
                result,
                interpretation,
            });
            
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }
}
