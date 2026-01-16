//! # Number Word Parser
//!
//! This crate provides a solution to the [number word problem](http://programmingpraxis.com/2014/07/25/number-words/),
//! which involves parsing a string of digits into all possible corresponding words based on a given mapping
//! (e.g., "1" -> 'A', "2" -> 'B', ..., "26" -> 'Z').
//!
//! It implements three different strategies to solve this problem, each with distinct performance characteristics
//! and algorithmic approaches. This serves as a pedagogical example of optimization in Rust.
//!
//! ## Strategies
//!
//! 1.  **Naive Recursion (`NaiveParser`)**: Directly constructs solutions using functional combinators. Simple to read but slow due to excessive allocation (`VecDeque`).
//! 2.  **Depth-First Search (`DfsParser`)**: Uses a recursive backtracking approach with a single mutable buffer. Highly memory-efficient and fast for small to medium inputs.
//! 3.  **Memoized Graph (`MemoizedParser`)**: A hybrid approach using Dynamic Programming. It precomputes the solution graph and solution counts to enable perfect pre-allocation and fast generation. Fastest for large, ambiguous inputs.

use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};
use thiserror::Error;

/// Errors that can occur during parsing.
#[derive(Error, Debug, PartialEq)]
pub enum NumberWordsError {
    /// The input contains a character that is not present in any key of the configuration.
    #[error("Input contains invalid character: '{0}'")]
    InvalidCharacter(char),
}

/// Configuration type representing the mapping from digit strings to characters.
/// Example: `vec![("1".to_string(), 'A'), ("2".to_string(), 'B'), ...]`
pub type Config = Vec<(String, char)>;

/// Generates the standard configuration mapping 1..26 to A..Z.
///
/// # Returns
///
/// A `Config` vector containing pairs like `("1", 'A')`, `("2", 'B')`, ..., `("26", 'Z')`.
pub fn default_config() -> Config {
    (b'A' ..= b'Z')
        .map(|b|
             ((b - b'A' + 1).to_string(),
              b as char))
        .collect()
}

/// A unified trait for all number parser implementations.
///
/// This trait allows different parsing strategies to be used interchangeably.
pub trait NumberParser {
    /// Parses a string of digits into all possible valid words.
    ///
    /// # Arguments
    ///
    /// *   `digits` - A string slice containing the digits to parse (e.g., "1234").
    ///
    /// # Returns
    ///
    /// A result containing a vector of valid decodings, or an error if validation fails.
    fn parse(&self, digits: &str) -> Result<Vec<String>, NumberWordsError>;
}

/// # Naive Parser
///
/// The original, functional implementation using recursive backtracking and `VecDeque`.
pub struct NaiveParser {
    max_lookahead: usize,
    table: HashMap<Vec<char>, char>,
    alphabet: HashSet<char>,
}

type WordInProgress = VecDeque<char>;

impl NaiveParser {
    /// Creates a new `NaiveParser` with the given configuration.
    pub fn new(config: &Config) -> Self {
        let mut alphabet = HashSet::new();
        for (key, _) in config {
            for c in key.chars() {
                alphabet.insert(c);
            }
        }

        Self {
            max_lookahead: config
                .iter()
                .map(|&(ref s, _)| s.len())
                .fold(0, cmp::max),
            table: config
                .iter()
                .map(|&(ref s, c)| (s.chars().collect(), c))
                .collect(),
            alphabet,
        }
    }

    fn validate(&self, digits: &str) -> Result<(), NumberWordsError> {
        for c in digits.chars() {
            if !self.alphabet.contains(&c) {
                return Err(NumberWordsError::InvalidCharacter(c));
            }
        }
        Ok(())
    }

    /// Internal recursive function using `VecDeque`.
    fn parse_list(&self, ds: &[char]) -> Vec<WordInProgress> {
        if ds.is_empty() {
            vec![VecDeque::new()]
        } else {
            let max_lookahead_index = cmp::min(self.max_lookahead, ds.len());
            let prefix = &ds[..max_lookahead_index];

            (1 ..= max_lookahead_index)
                .flat_map(|lookahead_index| {
                    let unparsed_index = cmp::min(lookahead_index, max_lookahead_index);
                    let token_slice = &prefix[..unparsed_index];

                    self.table.get(token_slice).map_or_else(
                        || vec![],
                        |&c| {
                            let unparsed = &ds[unparsed_index..];
                            self.parse_list(unparsed)
                                .into_iter()
                                .map(|mut s| {
                                    s.push_front(c);
                                    s
                                })
                                .collect::<Vec<WordInProgress>>()
                        })
                        .into_iter()
                })
                .collect()
        }
    }
}

impl NumberParser for NaiveParser {
    fn parse(&self, digits: &str) -> Result<Vec<String>, NumberWordsError> {
        self.validate(digits)?;
        let v = digits.chars().collect::<Vec<char>>();
        let parsed = self.parse_list(&v[..]);
        Ok(parsed
            .into_iter()
            .map(|char_list| char_list.into_iter().collect())
            .collect())
    }
}

/// # DFS Parser
///
/// A performance-optimized implementation using Depth-First Search (DFS) with a single mutable buffer.
pub struct DfsParser {
    max_lookahead: usize,
    table: HashMap<Vec<char>, char>,
    alphabet: HashSet<char>,
}

impl DfsParser {
    /// Creates a new `DfsParser` with the given configuration.
    pub fn new(config: &Config) -> Self {
        let mut alphabet = HashSet::new();
        for (key, _) in config {
            for c in key.chars() {
                alphabet.insert(c);
            }
        }

        Self {
            max_lookahead: config
                .iter()
                .map(|&(ref s, _)| s.len())
                .fold(0, cmp::max),
            table: config
                .iter()
                .map(|&(ref s, c)| (s.chars().collect(), c))
                .collect(),
            alphabet,
        }
    }

    fn validate(&self, digits: &str) -> Result<(), NumberWordsError> {
        for c in digits.chars() {
            if !self.alphabet.contains(&c) {
                return Err(NumberWordsError::InvalidCharacter(c));
            }
        }
        Ok(())
    }

    fn dfs(&self, chars: &[char], index: usize, current: &mut String, results: &mut Vec<String>) {
        if index == chars.len() {
            results.push(current.clone());
            return;
        }

        let max_len = cmp::min(self.max_lookahead, chars.len() - index);
        for len in 1..=max_len {
            let slice = &chars[index..index+len];
            if let Some(&c) = self.table.get(slice) {
                current.push(c);
                self.dfs(chars, index + len, current, results);
                current.pop();
            }
        }
    }
}

impl NumberParser for DfsParser {
    fn parse(&self, digits: &str) -> Result<Vec<String>, NumberWordsError> {
        self.validate(digits)?;
        let v = digits.chars().collect::<Vec<char>>();
        let mut results = Vec::new();
        let mut current = String::with_capacity(v.len());
        self.dfs(&v, 0, &mut current, &mut results);
        Ok(results)
    }
}

/// # Memoized Parser (Compiled Graph)
///
/// A highly optimized parser that uses a "Compile, Count, Generate" strategy.
pub struct MemoizedParser {
    max_lookahead: usize,
    table: HashMap<Vec<char>, char>,
    alphabet: HashSet<char>,
}

impl MemoizedParser {
    /// Creates a new `MemoizedParser` with the given configuration.
    pub fn new(config: &Config) -> Self {
        let mut alphabet = HashSet::new();
        for (key, _) in config {
            for c in key.chars() {
                alphabet.insert(c);
            }
        }

        Self {
            max_lookahead: config
                .iter()
                .map(|&(ref s, _)| s.len())
                .fold(0, cmp::max),
            table: config
                .iter()
                .map(|&(ref s, c)| (s.chars().collect(), c))
                .collect(),
            alphabet,
        }
    }

    fn validate(&self, digits: &str) -> Result<(), NumberWordsError> {
        for c in digits.chars() {
            if !self.alphabet.contains(&c) {
                return Err(NumberWordsError::InvalidCharacter(c));
            }
        }
        Ok(())
    }

    fn build_graph(&self, chars: &[char]) -> Vec<Vec<(char, usize)>> {
        let n = chars.len();
        let mut adj = vec![Vec::new(); n];

        for i in 0..n {
            let max_len = cmp::min(self.max_lookahead, n - i);
            for len in 1..=max_len {
                let slice = &chars[i..i+len];
                if let Some(&c) = self.table.get(slice) {
                    adj[i].push((c, i + len));
                }
            }
        }
        adj
    }

    fn count_solutions(&self, n: usize, adj: &[Vec<(char, usize)>]) -> usize {
        let mut dp = vec![0; n + 1];
        dp[n] = 1;

        for i in (0..n).rev() {
            for &(_, next_idx) in &adj[i] {
                dp[i] += dp[next_idx];
            }
        }
        dp[0]
    }

    fn generate(
        &self,
        adj: &[Vec<(char, usize)>],
        index: usize,
        current: &mut String,
        results: &mut Vec<String>,
    ) {
        if index == adj.len() {
            results.push(current.clone());
            return;
        }

        for &(c, next_idx) in &adj[index] {
            current.push(c);
            self.generate(adj, next_idx, current, results);
            current.pop();
        }
    }
}

impl NumberParser for MemoizedParser {
    fn parse(&self, digits: &str) -> Result<Vec<String>, NumberWordsError> {
        self.validate(digits)?;
        let v = digits.chars().collect::<Vec<char>>();
        let n = v.len();

        let adj = self.build_graph(&v);
        let count = self.count_solutions(n, &adj);
        
        let mut results = Vec::with_capacity(count);
        let mut current = String::with_capacity(n);
        self.generate(&adj, 0, &mut current, &mut results);
        
        Ok(results)
    }
}

pub type Parser = NaiveParser;

#[cfg(test)]
mod test {
    use super::{default_config, NumberParser, NaiveParser, DfsParser, MemoizedParser, NumberWordsError};
    use std::collections::HashSet;

    fn run_common_tests<P: NumberParser>(parser: &P) {
        let cases: Vec<(&str, Vec<&str>)> = vec![
            ("1234", vec!["ABCD", "AWD", "LCD"]),
            ("", vec![""]),
            ("1", vec!["A"]),
            ("15", vec!["AE", "O"]),
            ("111", vec!["AAA", "AK", "KA"]),
            ("20", vec!["T"]),
            // Valid digit (0 is a digit in general sense, but here "0" is not in mapping)
            // But wait! If "0" is not in config, it is NOT in alphabet!
            // So "0" should return InvalidCharacter('0') if 0 is not in any key.
            // The default config is 1..26. '0' appears in "10", "20", "30"?
            // "10" is 'J'. So '0' IS in the alphabet.
            // So "0" should be Valid Input but No Solution.
            ("0", vec![]),
            ("99", vec!["II"]),
            ("10", vec!["J"]),
            ("01", vec![]),
        ];

        for (input, expected) in cases {
            let result = parser.parse(input);
            assert!(result.is_ok(), "Failed on valid input: '{}', err: {:?}", input, result.err());
            
            let actual = result.unwrap();
            let expected_set: HashSet<&str> = expected.iter().cloned().collect();
            let actual_set_owned: HashSet<String> = actual.into_iter().collect();
            let actual_set: HashSet<&str> = actual_set_owned.iter().map(|s| s.as_str()).collect();

            assert_eq!(actual_set, expected_set, "Failed on input: '{}'", input);
        }
    }

    #[test]
    fn test_invalid_chars() {
        let config = default_config();
        let parser = NaiveParser::new(&config);
        
        // 'A' is not in 1..26 keys. keys are "1".."26".
        // Wait, keys are digits. Input "A" -> Invalid.
        match parser.parse("A") {
            Err(NumberWordsError::InvalidCharacter(c)) => assert_eq!(c, 'A'),
            _ => panic!("Expected InvalidCharacter('A')"),
        }
        
        // ' ' (space)
        match parser.parse("1 2") {
            Err(NumberWordsError::InvalidCharacter(c)) => assert_eq!(c, ' '),
            _ => panic!("Expected InvalidCharacter(' ')"),
        }
    }

    #[test]
    fn naive_suite() {
        run_common_tests(&NaiveParser::new(&default_config()));
    }

    #[test]
    fn dfs_suite() {
        run_common_tests(&DfsParser::new(&default_config()));
    }

    #[test]
    fn memoized_suite() {
        run_common_tests(&MemoizedParser::new(&default_config()));
    }
}
