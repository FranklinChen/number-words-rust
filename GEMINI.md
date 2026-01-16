# Number Words Rust

## Project Overview

`number_words` (v0.0.4) is a Rust crate that solves a [number word problem](http://programmingpraxis.com/2014/07/25/number-words/). It parses a string of digits into all possible corresponding words based on a given mapping (e.g., 1 -> 'A', 2 -> 'B', ..., 26 -> 'Z').

The core logic is implemented in `src/lib.rs` using a recursive parser with lookahead to handle ambiguous encodings (e.g., "123" could be "ABC", "AW", or "LC").

## Building and Running

This project uses standard Cargo commands for building and testing.

### Build
To build the project in release mode:
```bash
cargo build --release
```

### Test
To run the test suite (which includes a basic "1234" -> "ABCD", "AWD", "LCD" test case):
```bash
cargo test
```

### Check
To quickly check for compilation errors:
```bash
cargo check
```

### Documentation
To generate documentation:
```bash
cargo doc --no-deps
```

## Codebase Structure

*   **`Cargo.toml`**: Project configuration, dependencies (including `divan` for benchmarking).
*   **`src/lib.rs`**: The main library file containing:
    *   `Config`: Type alias for the mapping configuration.
    *   `default_config()`: Generates the standard A=1..Z=26 mapping.
    *   `NumberParser`: A trait providing a unified interface for parsing.
    *   `NaiveParser`: The original implementation using `VecDeque`.
    *   `DfsParser`: A performance-optimized implementation using DFS to avoid intermediate allocations.
    *   `MemoizedParser`: An implementation using a sub-problem cache for efficiency on highly ambiguous inputs.
    *   `Parser`: Type alias to `NaiveParser` for backward compatibility.
    *   `mod test`: Unit tests for all implementations.
*   **`benches/benchmark.rs`**: Performance comparison benchmarks using `divan`.

## Development Conventions

*   **Language:** Rust (Edition 2024).
*   **Style:** Idiomatic Rust style is expected (verified via `cargo check` and standard formatting).
*   **CI:** Continuous Integration is set up via GitHub Actions in `.github/workflows/ci.yml`.
*   **License:** Dual-licensed under MIT or Apache-2.0.
