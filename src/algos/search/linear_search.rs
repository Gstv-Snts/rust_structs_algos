pub fn linear_search<T: Ord + Clone + Send + Sync + Copy>(vec: &Vec<T>, target: T) -> Option<T> {
    for i in 0..vec.len() {
        if vec[i] == target {
            return Some(vec[i].clone());
        }
    }
    None
}
#[cfg(test)]
mod test {
    #[test]
    fn linear_search_test() {
        let v = vec![3, 2, 10, 4, 23, 7, 11, 12];
        assert_eq!(super::linear_search(&v, 7), Some(7));
        assert_eq!(super::linear_search(&v, 1), None);
    }
}
