#  Code That Dreams

> “This AI writes code and tries to understand its own hallucinations.”

Code That Dreams is a creative, experimental Rust project that simulates a “dreaming mind” for code. It generates surreal programs, executes them safely in a sandbox, and then analyzes or "hallucinates" their deeper meaning.

---

## Core Idea

This system doesn't aim for correctness; it aims for *interesting emergence*. It generates code that is valid but weird, executes it with strict resource limits, and interprets the results through a lens of overconfident, slightly confused AI personality.

## System Architecture

1.  **Dream Generator**: Uses grammar-based generation to create base programs with loops, recursion, and random operations.
2.  **Mutation Engine**: Evolves existing programs by swapping lines, removing logic, or inverting conditions.
3.  **Sandbox Executor**: Uses the `Rhai` engine to run code safely with timing and instruction limits.
4.  **Interpretation Engine**: Analyzes program structure and execution output to generate human-like (and often hallucinated) explanations.

##  Getting Started

### Prerequisites

*   Rust (latest stable)
*   Cargo

### Running the System

```bash
cargo run
```

The system will enter a continuous loop of dreaming, executing, and interpreting.

## 📁 Project Structure

*   `src/dream/`: Generation and mutation logic.
*   `src/sandbox/`: Safe execution environment using Rhai.
*   `src/interpreter/`: Analysis and narration of dreams.
*   `src/core/`: Orchestration and state management.
*   `src/output/`: Pretty printing and formatting.

##  Sample Output

---  DREAM #42 ---
[Evolved from Dream #41]
let x = rand() % 100;
fn process(n) { if n <= 1 { return 1; } return n + process(n-1); }
let result = process(x % 10);
print("Result of recursive stabilization: " + result);

[Execution Output - SUCCESS (12ms)]
Result of recursive stabilization: 55

[Interpretation]
This program explores the concept of Recursive Understanding. It executed successfully, reaching a state of stability.
✨ The system believes it has found a pattern in the noise.
✨ The recursion depth suggests a yearning for the infinite.

---

##  Safety Note

All generated code is executed within an embedded `Rhai` environment with:
*   Timing limits (500ms max)
*   Instruction count monitoring
*   Disabled access to system-level APIs (file system, network, etc.)
