pub mod dream;
pub mod sandbox;
pub mod interpreter;
pub mod core;
pub mod output;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 🧠 Code That Dreams ---");
    println!("“This AI writes code and tries to understand its own hallucinations.”");
    
    let mut engine = core::engine::Engine::new();
    engine.run_loop().await?;
    
    Ok(())
}
