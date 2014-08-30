use std::iter::range_inclusive;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::DList;

pub type Config = Vec<(String, char)>;

// Instead of Vec<char>, in order to use O(1) push_front.
// DList is good for mutation.
// Vec<char> was reasonable but required building in reverse.
type WordInProgress = DList<char>;

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
    /// Note the use of move_iter.
    pub fn parse(&self, digits: &str) -> Vec<String> {
        // It is convenient to use char slices.
        let v: Vec<char> = digits.chars().collect();
        let parsed = self.parse_list(v.as_slice());
        parsed
            .move_iter()
            .map(|char_list| {
                let chars: Vec<char> = char_list
                    .move_iter()
                    .collect();
                String::from_chars(chars.as_slice())
            })
            .collect()
    }

    /// Recursive.
    /// Note the use of flat_map and move_iter to avoid redundant
    /// allocation and copying of vectors.
    fn parse_list(&self, ds: &[char]) -> Vec<WordInProgress> {
        match ds {
            [] => vec![DList::new()],
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
                        let token = Vec::from_slice(token_slice);

                        self.table.find(&token).map_or_else(
                            || vec![],
                            |&c| {
                                let unparsed = ds.slice_from(unparsed_index);

                                // Mutate recursive result in place,
                                // instead of mapping.
                                let mut rest_parsed = self.parse_list(unparsed);
                                for s in rest_parsed.mut_iter() {
                                    //TODO waiting for this to appear
                                    //s.push_front(c)
                                    let mut new_front = DList::new();
                                    new_front.push(c);
                                    s.prepend(new_front);
                                }
                                rest_parsed
                            })
                            .move_iter()
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
