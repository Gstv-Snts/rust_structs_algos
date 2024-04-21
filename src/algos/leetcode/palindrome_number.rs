pub fn is_palindrome(x: i32) -> bool {
    if &x.to_string() == &x.to_string().chars().rev().collect::<String>() {
        return true;
    }
    false
}

mod test {
    use crate::algos::leetcode::is_palindrome;

    #[test]
    fn is_palindrome_1() {
        assert_eq!(is_palindrome(121), true)
    }
    #[test]
    fn is_palindrome_2() {
        assert_eq!(is_palindrome(-121), false)
    }
    #[test]
    fn is_palindrome_3() {
        assert_eq!(is_palindrome(10), false)
    }
}
