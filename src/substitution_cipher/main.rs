use std::vec;

static SUBS: [char; 26] = [
    'w', 'a', 'c', 'f', 'g', 'z', 'm', 'i', 's', 'j', 'k', 'l', 'n', 'p', 'q', 'r', 't', 'u', 'v',
    'b', 'x', 'e', 'd', 'o', 'y', 'h',
];

static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn substitute(text: &str) -> String {
    text.chars()
        .map(|c: char| SUBS[ALPHABET.iter().position(|&s| s == c).unwrap()])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::substitute;

    #[test]
    fn correct_results() {
        assert_eq!(substitute(&"abc".to_string()), "wac", "wrong result");
        assert_eq!(substitute(&"xyz".to_string()), "oyh", "wrong result");
    }
}
