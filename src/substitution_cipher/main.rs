fn main(text: &str) {
    let subs = vec!['a'];
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn test_something() {
        main(&"abc".to_string());
    }
}
