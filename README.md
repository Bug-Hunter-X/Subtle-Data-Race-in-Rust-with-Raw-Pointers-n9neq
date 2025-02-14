# Subtle Data Race in Rust with Raw Pointers
This repository demonstrates a subtle data race that can occur in Rust when using raw pointers and modifying vectors.  The code appears correct at first glance but introduces undefined behavior.

The `bug.rs` file contains the buggy code, while `bugSolution.rs` provides a corrected version using safe Rust techniques.

This example highlights the importance of understanding memory management and using Rust's safe abstractions to prevent data races and maintain memory safety.