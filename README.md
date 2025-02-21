# Rust Out-of-Bounds Access Error

This repository demonstrates a common error in Rust: accessing an array or vector element using an index that is out of bounds.  The `bug.rs` file contains code that will panic at runtime if an invalid index is used.  The `bugSolution.rs` file shows how to handle this error gracefully using the `get()` method and a match statement to check for `None`. 