# Number Words Rust

[![Crates.io](https://img.shields.io/crates/v/number_words.svg)](https://crates.io/crates/number_words)
[![Docs](https://docs.rs/number_words/badge.svg)](https://docs.rs/number_words/)
[![CI](https://github.com/FranklinChen/number-words-rust/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/FranklinChen/number-words-rust/actions/workflows/ci.yml)

A Rust crate that explores different algorithmic strategies to solve the [number word problem](http://programmingpraxis.com/2014/07/25/number-words/).

The problem involves parsing a string of digits into all possible corresponding words based on a given mapping (e.g., `1` -> 'A', `2` -> 'B', ... `26` -> 'Z').
For example, the input `"123"` can be parsed as:
- "ABC" (1, 2, 3)
- "AW" (1, 23)
- "LC" (12, 3)

## Implementations

This crate provides three different parser implementations, each with distinct performance characteristics, serving as a pedagogical example of optimization in Rust.

### 1. Naive Parser (`NaiveParser`)
The original implementation using a functional approach with `VecDeque`.
- **Pros**: Purely functional, demonstrates iterator combinators.
- **Cons**: Extremely slow for large inputs due to excessive allocation and copying ($O(N^2)$ copying behavior).

### 2. DFS Parser (`DfsParser`)
A performance-optimized implementation using Depth-First Search with a single mutable buffer.
- **Pros**: Very fast due to cache locality and minimal heap allocation.
- **Cons**: Recomputes subproblems if the graph has overlapping paths.

### 3. Memoized Parser (`MemoizedParser`)
A hybrid approach using Dynamic Programming to precompute the solution graph and solution counts.
- **Pros**: The fastest implementation for large, highly ambiguous inputs. It pre-allocates exact memory for results, preventing costly reallocations.
- **Cons**: Slightly more complex implementation logic.

## Usage

```rust
use number_words::{default_config, NumberParser, DfsParser};

fn main() {
    // 1. Create a configuration (standard A=1..Z=26)
    let config = default_config();

    // 2. Instantiate a parser (e.g., DfsParser)
    let parser = DfsParser::new(&config);

    // 3. Parse a digit string
    let results = parser.parse("1234").expect("Valid input");

    // Output: ["ABCD", "AWD", "LCD"]
    for word in results {
        println!("{}", word);
    }
}
```

## Development

### Build
To build the project in release mode:
```bash
cargo build --release
```

### Test
To run the test suite:
```bash
cargo test
```

### Benchmark
This project uses `divan` for benchmarking. To run benchmarks:
```bash
cargo bench
```

## Benchmarks

Benchmarks were run on an ambiguous input (`"1"` repeated $N$ times) which triggers the worst-case exponential growth of solutions.

| Input Length (N) | NaiveParser | DfsParser | MemoizedParser | Speedup (Memo vs Naive) |
| :--- | :--- | :--- | :--- | :--- |
| **10** | 30.1 µs | 7.6 µs | 3.0 µs | **10x** |
| **20** | 4.4 ms | 0.8 ms | 0.25 ms | **17x** |
| **25** | 58.3 ms | 8.6 ms | 2.7 ms | **21x** |
| **30** | N/A | 100.3 ms | 32.3 ms | - |

*   **NaiveParser**: Struggles with overhead from repeated `VecDeque` allocations.
*   **DfsParser**: significantly faster by using a single stack, but still does redundant work.
*   **MemoizedParser**: The clear winner. By pre-calculating the graph and solution counts, it avoids all redundant allocations and writes directly to the final memory location.

## AI-Assisted Development

This project was significantly improved with the assistance of **Gemini**, an AI by Google. Gemini was used to:
- Refactor the codebase for more principled error handling using `thiserror`.
- Expand the test suite to cover critical edge cases and invalid inputs.
- Implement and document high-performance parsing strategies (`DfsParser`, `MemoizedParser`).
- Generate comprehensive benchmarks and technical documentation.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.