use std::iter::range_inclusive;
use std::cmp::min;
use std::collections::HashMap;

pub type Config = Vec<(String, char)>;

type WordInProgress = Vec<char>;

type WordsInProgress = Vec<WordInProgress>;

pub fn default_config() -> Config {
    range_inclusive(b'A', b'Z')
        .map(|b|
             ((b - b'A' + 1).to_string(),
              b as char))
        .collect()
}

/// Utility function.
/// Unlike standard library split_at, always stays within bounds.
fn split_at_within<T>(i: uint, xs: &[T]) -> (&[T], &[T]) {
    xs.split_at(min(i, xs.len()))
}

pub struct Parser {
    max_chunk: uint,
    table: HashMap<Vec<char>, char>
}

impl Parser {
    pub fn new(config: &Config) -> Parser {
        Parser {
            max_chunk: config
                .iter()
                .map(|&(ref s, _)|
                     s.len())
                .max_by(|&n| n)
                .unwrap_or(0),
            table: config
                .iter()
                .map(|&(ref s, c)|
                     (s.as_slice().chars().collect(), c))
                .collect()
        }
    }

    /// Entry point.
    /// Internally, use sequences rather than strings.
    /// Note the use of move_iter.
    pub fn parse(&self, digits: &str) -> Vec<String> {
        // It is convenient to use char slices.
        let v: Vec<char> = digits.chars().collect();
        let parsed = self.parse_list(v.as_slice());
        parsed
            .move_iter()
            .map(|reversed_chars| {
                let chars: WordInProgress = reversed_chars
                    .move_iter()
                    .rev()
                    .collect();
                String::from_chars(chars.as_slice())
            })
            .collect()
    }

    /// Note the use of flat_map and move_iter to avoid needless append.
    fn parse_list(&self, ds: &[char]) -> WordsInProgress {
        match ds {
            [] => vec![vec![]],
            _ => {
                // Split into possible prefix/suffix halves.
                let (prefix, suffix) = split_at_within(self.max_chunk, ds);

                range_inclusive(1u, prefix.len())
                    .flat_map(|i| {
                        let (digits, unparsed) = split_at_within(i, prefix);
                        self.try_parse(digits, unparsed, suffix)
                            .move_iter()
                    })
                    .collect()
            }
        }
    }
    
    /// Append first char to the end of each WordInProgress for efficiency.
    /// At the very end of parse, we will reverse each to a String.
    fn try_parse(&self,
                 digits: &[char],
                 unparsed: &[char],
                 suffix: &[char]) -> WordsInProgress {
        // TODO waiting for master for map_or_else
        match self.table.find(&Vec::from_slice(digits)) {
            None => vec![],
            Some(&c) => {
                let rest = Vec::from_slice(unparsed).append(suffix);
                let rest_parsed = self.parse_list(rest.as_slice());
                rest_parsed
                    .move_iter()
                    .map(|s| s.append_one(c))
                    .collect()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::default_config;
    use super::Parser;
    use std::collections::hashmap::HashSet;

    #[test]
    fn it_works() {
        let parser = Parser::new(&default_config());

        let expected = vec!["ABCD", "AWD", "LCD"];
        let expected_set: HashSet<String> = expected
            .move_iter()
            .map(|s| s.to_string())
            .collect();

        let actual = parser.parse("1234");
        let actual_set: HashSet<String> = actual
            .move_iter()
            .collect();
        assert_eq!(actual_set, expected_set)
    }
}
