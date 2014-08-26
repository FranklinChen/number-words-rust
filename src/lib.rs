// TODO Look into using mutation with DList for O(1) append.
// But then what about parallelism?

use std::iter::range_inclusive;
use std::char;
use std::collections::HashMap;

pub type Config = Vec<(String, char)>;

fn default_config() -> Config {
    range_inclusive(b'A', b'Z')
        .map(|b| ((b - b'A' + 1).to_string(),
                  char::from_u32(b as u32).unwrap()))
        .collect()
}

pub struct Parser {
    max_chunk: uint,
    table: HashMap<String, char>
}

impl Parser {
    pub fn new(config: &Config) -> Parser {
        Parser {
            max_chunk: config
                .iter()
                .map(|&(ref s, _)| s.len())
                .max_by(|&n| n)
                .unwrap_or(0),
            table: config
                .iter()
                .map(|&(ref s, c)| (s.clone(), c))
                .collect()
        }
    }

    /// Entry point.
    /// Internally, use sequences rather than strings.
    /// Note the use of move_iter.
    pub fn parse(&self, digits: &str) -> Vec<String> {
        // It is convenient to use char slices.
        let v: Vec<char> = digits.chars().collect();
        self.parse_list(v.as_slice())
            .move_iter()
            .map(|reversed_chars| {
                let chars: Vec<char> = reversed_chars
                    .move_iter()
                    .rev()
                    .collect();
                String::from_chars(chars.as_slice())
            })
            .collect()
    }

    /// Note the append
    fn parse_list(&self, ds: &[char]) -> Vec<Vec<char>> {
        match ds {
            [] => vec![vec![]],
            [d0, ..ds0] => {
                let first = self.try_parse(
                    self.try_lookup(&String::from_chars([d0])),
                    ds0);
                match ds0 {
                    [] => first,
                    [d1, ..ds1] => {
                        let second = self.try_parse(
                            self.try_lookup(&String::from_chars([d0, d1])),
                            ds1);
                        second.append(first.as_slice())
                    }
                }
            }
        }
    }
    
    /// Append first char to the end of each Vec<char> for efficiency.
    /// At the very end of parse, we will reverse each Vec<char> to a String.
    fn try_parse(&self, c_opt: Option<char>, ds: &[char]) -> Vec<Vec<char>> {
        match c_opt {
            None => vec![],
            Some(c) => {
                self.parse_list(ds)
                    .move_iter()
                    .map(|s| s.append_one(c))
                    .collect()
            }
        }
    }
    
    fn try_lookup(&self, s: &String) -> Option<char> {
        self.table.find(s).map(|c| *c)
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
