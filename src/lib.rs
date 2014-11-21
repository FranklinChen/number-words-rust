//! Solve a [number word problem](http://programmingpraxis.com/2014/07/25/number-words/).

use std::iter::range_inclusive;
use std::cmp::min;
use std::collections::HashMap;

pub type Config = Vec<(String, char)>;

/// Reversed word in progress. Reversed because appending a char
/// to end is O(1).
type WordInProgress = Vec<char>;

pub fn default_config() -> Config {
    range_inclusive(b'A', b'Z')
        .map(|b|
             ((b - b'A' + 1).to_string(),
              b as char))
        .collect()
}

pub struct Parser {
    max_lookahead: uint,
    table: HashMap<Vec<char>, char>
}

impl Parser {
    pub fn new(config: &Config) -> Parser {
        Parser {
            max_lookahead: config
                .iter()
                .map(|&(ref s, _)|
                     s.len())   // get string lengths
                .max_by(|&n| n)
                .unwrap_or(0),
            table: config
                .iter()
                .map(|&(ref s, c)| // String -> Vec<char>
                     (s.as_slice().chars().collect(), c))
                .collect()
        }
    }

    /// Entry point.
    /// Internally, get out of string early, to use chars instead.
    /// Note the use of into_iter.
    pub fn parse(&self, digits: &str) -> Vec<String> {
        // It is convenient to use char slices.
        let v = digits.chars().collect::<Vec<char>>();
        let parsed = self.parse_list(v.as_slice());
        parsed
            .into_iter()
            .map(|char_list| {
                let chars = char_list
                    .into_iter()
                    .rev()
                    .collect::<Vec<char>>();
                String::from_chars(chars.as_slice())
            })
            .collect()
    }

    /// Recursive.
    /// Note the use of flat_map and into_iter to avoid redundant
    /// allocation and copying of vectors.
    fn parse_list(&self, ds: &[char]) -> Vec<WordInProgress> {
        match ds {
            [] => vec![vec![]],
            _ => {
                // Try all parses up to the maximum lookahead.
                let max_lookahead_index = min(self.max_lookahead, ds.len());
                let prefix = ds.slice_to(max_lookahead_index);

                range_inclusive(1u, max_lookahead_index)
                    .flat_map(|lookahead_index| {
                        // Split into possible parsed/unparsed configurations.
                        let unparsed_index = min(lookahead_index,
                                                 max_lookahead_index);

                        // Actual token to look up.
                        let token_slice = prefix.slice_to(unparsed_index);
                        let token = token_slice.to_vec();

                        self.table.get(&token).map_or_else(
                            || vec![],
                            |&c| {
                                let unparsed = ds.slice_from(unparsed_index);

                                self.parse_list(unparsed)
                                    .into_iter()
                                    .map(|mut s| {
                                        // mutate for efficiency
                                        // pushing to end is efficient
                                        s.push(c);
                                        s
                                    })
                                    .collect::<Vec<WordInProgress>>()
                            }
                        )
                        .into_iter()
                    })
                    .collect()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::default_config;
    use super::Parser;
    use std::collections::HashSet;

    #[test]
    fn it_works() {
        let parser = Parser::new(&default_config());

        let expected = ["ABCD", "AWD", "LCD"];
        let expected_set = expected
            .iter()
            .map(|&s| s)
            .collect::<HashSet<&str>>();

        let actual = parser.parse("1234");
        let actual_set = actual
            .iter()
            .map(|s| s.as_slice())
            .collect::<HashSet<&str>>();
        assert_eq!(actual_set, expected_set)
    }
}
