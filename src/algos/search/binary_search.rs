pub fn binary_search<T: Ord + Clone>(vec: &Vec<T>, target: T) -> Option<T> {
    let mut left = 0;
    let mut right = vec.len() - 1;
    let mut middle = 0;
    while middle < right {
        middle = (right + left) / 2;
        if vec[left] == target {
            return Some(vec[left].clone());
        }
        if vec[right] == target {
            return Some(vec[right].clone());
        }
        if vec[middle] == target {
            return Some(vec[middle].clone());
        }
        if target > vec[middle] {
            left = middle
        } else {
            right = middle
        }
    }
    None
}

#[cfg(test)]
mod test {
    #[test]
    fn binary_search() {
        let v = vec![3, 4, 1, 10, 23, 40, 2];
        assert_eq!(super::binary_search(&v, 40), Some(40))
    }
}
