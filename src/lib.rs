//! Solve a [number word problem](http://programmingpraxis.com/2014/07/25/number-words/).

use std::cmp;
use std::collections::HashMap;
use std::collections::VecDeque;

pub type Config = Vec<(String, char)>;

/// Word in progress, constructed back to front.
type WordInProgress = VecDeque<char>;

pub fn default_config() -> Config {
    (b'A' ..= b'Z')
        .map(|b|
             ((b - b'A' + 1).to_string(),
              b as char))
        .collect()
}

pub struct Parser {
    max_lookahead: usize,
    table: HashMap<Vec<char>, char>
}

impl Parser {
    pub fn new(config: &Config) -> Parser {
        Parser {
            max_lookahead: config
                .iter()
                .map(|&(ref s, _)|
                     s.len())   // get string lengths
                .fold(0, cmp::max),
            table: config
                .iter()
                .map(|&(ref s, c)| // String -> Vec<char>
                     (s.chars().collect(), c))
                .collect()
        }
    }

    /// Entry point.
    /// Internally, get out of string early, to use chars instead.
    /// Note the use of into_iter.
    pub fn parse(&self, digits: &str) -> Vec<String> {
        // It is convenient to use char slices.
        let v = digits.chars().collect::<Vec<char>>();
        let parsed = self.parse_list(&v[..]);
        parsed
            .into_iter()
            .map(|char_list| {
                char_list
                    .into_iter()
                    .collect()
            })
            .collect()
    }

    /// Recursive.
    /// Note the use of flat_map and into_iter to avoid redundant
    /// allocation and copying of vectors.
    fn parse_list(&self, ds: &[char]) -> Vec<WordInProgress> {
        if ds.is_empty() {
            vec![VecDeque::new()]
        } else {
            // Try all parses up to the maximum lookahead.
            let max_lookahead_index = cmp::min(self.max_lookahead, ds.len());
            let prefix = &ds[..max_lookahead_index];

            (1 ..= max_lookahead_index)
                .flat_map(|lookahead_index| {
                    // Split into possible parsed/unparsed configurations.
                    let unparsed_index = cmp::min(lookahead_index,
                                                  max_lookahead_index);

                    // Actual token to look up.
                    let token_slice = &prefix[..unparsed_index];

                    self.table.get(token_slice).map_or_else(
                        || vec![],
                        |&c| {
                            let unparsed = &ds[unparsed_index..];

                            self.parse_list(unparsed)
                                .into_iter()
                                .map(|mut s| {
                                    // mutate for efficiency
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
            .cloned()
            .collect::<HashSet<&str>>();

        let actual = parser.parse("1234");
        let actual_set = actual
            .iter()
            .map(|s| &s[..])
            .collect::<HashSet<&str>>();

        assert_eq!(actual_set, expected_set)
    }
}
