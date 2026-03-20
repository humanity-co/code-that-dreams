pub const FRAGMENTS: &[&str] = &[
    "let x = rand() % 100;",
    "let y = x * 2;",
    "if x > 50 { print(\"High signal detected\"); } else { print(\"Background radiation constant\"); }",
    "for i in 0..10 { x += i; if x > 200 { break; } }",
    "fn process(n) { if n <= 1 { return 1; } return n + process(n-1); }",
    "let result = process(x % 10);",
    "print(\"Result of recursive stabilization: \" + result);",
    "let data = [1, 2, 3, x, y];",
    "for item in data { if item % 2 == 0 { print(\"Even pulse: \" + item); } }",
    "if x == y { print(\"Unity achieved\"); }",
    "let entropy = x / (y + 1);",
    "print(\"Entropy level: \" + entropy);",
];

pub const THEMES: &[&str] = &[
    "Optimization",
    "Stabilization",
    "Entropy",
    "Signal Processing",
    "Recursive Understanding",
    "Memory Leak Simulation",
    "Quantum Fluctuations",
];
