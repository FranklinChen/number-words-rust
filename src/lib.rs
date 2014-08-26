// TODO Look into using mutation with DList for O(1) append.
// But then what about parallelism?
// TODO Use TrieSet for lookup in general

/// Entry point.
/// Internally, use sequences rather than strings.
/// Note the use of move_iter.
pub fn parse(digits: &str) -> Vec<String> {
    // It is convenient to use char slices.
    let v: Vec<char> = digits.chars().collect();
    parse_list(v.as_slice())
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
fn parse_list(ds: &[char]) -> Vec<Vec<char>> {
    match ds {
        [] => vec![vec![]],
        [d0, ..ds0] => {
            let first = try_parse(
                try_lookup(String::from_chars([d0]).as_slice()),
                ds0);
            match ds0 {
                [] => first,
                [d1, ..ds1] => {
                    let second = try_parse(
                        try_lookup(String::from_chars([d0, d1]).as_slice()),
                        ds1);
                    second.append(first.as_slice())
                }
            }
        }
    }
}

/// Append first char to the end of each Vec<char> for efficiency.
/// At the very end of parse, we will reverse each Vec<char> to a String.
fn try_parse(c_opt: Option<char>, ds: &[char]) -> Vec<Vec<char>> {
    match c_opt {
        None => vec![],
        Some(c) => {
            parse_list(ds)
                .move_iter()
                .map(|s| s.append_one(c))
                .collect()
        }
    }
}

fn try_lookup(s: &str) -> Option<char> {
    match s {
        "1" => Some('A'),
        "2" => Some('B'),
        "3" => Some('C'),
        "4" => Some('D'),
        "5" => Some('E'),
        "6" => Some('F'),
        "7" => Some('G'),
        "8" => Some('H'),
        "9" => Some('I'),
        "10" => Some('J'),
        "11" => Some('K'),
        "12" => Some('L'),
        "13" => Some('M'),
        "14" => Some('N'),
        "15" => Some('O'),
        "16" => Some('P'),
        "17" => Some('Q'),
        "18" => Some('R'),
        "19" => Some('S'),
        "20" => Some('T'),
        "21" => Some('U'),
        "22" => Some('V'),
        "23" => Some('W'),
        "24" => Some('X'),
        "25" => Some('Y'),
        "26" => Some('Z'),
        _ => None
    }
}

#[cfg(test)]
mod test {
    use super::parse;
    use std::collections::hashmap::HashSet;

    #[test]
    fn it_works() {
        let expected = vec!["ABCD", "AWD", "LCD"];
        let expected_set: HashSet<String> = expected
            .move_iter()
            .map(|s| s.to_string())
            .collect();

        let actual = parse("1234");
        let actual_set: HashSet<String> = actual
            .move_iter()
            .collect();
        assert_eq!(actual_set, expected_set)
    }
}
