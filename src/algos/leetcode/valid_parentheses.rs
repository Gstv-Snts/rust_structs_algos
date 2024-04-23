pub fn is_valid(s: String) -> bool {
    let mut v = vec![];
    for char in s.chars() {
        match char {
            '(' => v.push(')'),
            '[' => v.push(']'),
            '{' => v.push('}'),
            '}' | ')' | ']' if Some(char) != v.pop() => return false,
            _ => (),
        }
    }
    v.is_empty()
}
mod test {
    #[test]
    fn valid_parenthesis() {
        assert_eq!(super::is_valid("()".to_string()), true);
        assert_eq!(super::is_valid("()[]{}".to_string()), true);
        assert_eq!(super::is_valid("(]".to_string()), false);
        assert_eq!(super::is_valid("(){}}{".to_string()), false);
        assert_eq!(super::is_valid("{[]}".to_string()), false);
    }
}
