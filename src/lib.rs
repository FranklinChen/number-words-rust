// TODO Look into using mutation with DList for O(1) append.
// But then what about parallelism?
// TODO Use Chars iterator?
// TODO Use TrieSet for lookup in general

pub fn parse(digits: &str) -> Vec<String> {
    let v: Vec<char> = digits.chars().collect();
    parse_list(v.as_slice())
}

fn two_chars_to_string(d0: char, d1: char) -> String {
    let mut d0_d1 = String::with_capacity(2);
    d0_d1.push_char(d0);
    d0_d1.push_char(d1);
    d0_d1
}

/// Note the append
fn parse_list(ds: &[char]) -> Vec<String> {
    match ds {
        [] => vec!["".to_string()],
        [d0, ..ds0] => {
            let first = try_parse(
                try_lookup(String::from_char(1, d0).as_slice()),
                ds0);
            match ds0 {
                [] => first,
                [d1, ..ds1] => {
                    let second = try_parse(
                        try_lookup(two_chars_to_string(d0, d1).as_slice()),
                        ds1);
                    second.append(first.as_slice())
                }
            }
        }
    }
}

/// TODO: fix horribly inefficient append to a single char
fn try_parse(c_opt: Option<char>, ds: &[char]) -> Vec<String> {
    match c_opt {
        None => vec![],
        Some(c) => {
            let remainder = parse_list(ds);
            remainder.iter()
                .map(|s| String::from_char(1, c).append(s.as_slice()))
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
        let expected_set: HashSet<String> = expected.move_iter()
            .map(|s| s.to_string())
            .collect();

        let actual = parse("1234");
        let actual_set: HashSet<String> = actual
            .move_iter()
            .collect();
        assert_eq!(actual_set, expected_set)
    }
}
