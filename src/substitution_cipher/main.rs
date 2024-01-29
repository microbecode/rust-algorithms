use std::vec;

static SUBS: [char; 26] = [
    'w', 'a', 'c', 'f', 'g', 'z', 'm', 'i', 's', 'j', 'k', 'l', 'n', 'p', 'q', 'r', 't', 'u', 'v',
    'b', 'x', 'e', 'd', 'o', 'y', 'h',
];

static ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

//static SUBS: Vec<char> = vec!['a'];
fn substitute(text: &str) {
    let a: String = text
        .chars()
        .map(|c: char| SUBS[ALPHABET.iter().position(|&s| s == c).unwrap()])
        .collect();
    println!("{:?}", a);
}

#[cfg(test)]
mod tests {
    use super::substitute;

    #[test]
    fn test_something() {
        substitute(&"abc".to_string());
    }
}
