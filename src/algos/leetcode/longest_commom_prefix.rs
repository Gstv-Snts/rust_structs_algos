pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs[0].clone();
    }
    let mut prefix = "";
    for i in 0..strs[0].len() + 1 {
        for str in &strs {
            if strs[0].get(..i) != str.get(..i) {
                return prefix.to_string();
            }
        }
        prefix = strs[0].get(..i).unwrap();
    }
    prefix.to_string()
}

mod test {
    use crate::algos::leetcode::longest_common_prefix;

    #[test]
    fn longest_common_prefix_1() {
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
        assert_eq!(longest_common_prefix(vec!["a".to_string(),]), "a");
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_string(),
                "flower".to_string(),
                "flower".to_string(),
                "flower".to_string()
            ]),
            "flower"
        )
    }
}
