pub fn bubble_sort<T: Ord + Clone, const COUNT: usize>(v: &mut [T; COUNT]) {
    for i in 0..v.len() {
        for j in 0..v.len() {
            if v[i] < v[j] {
                let tmp = v[j].clone();
                v[j] = v[i].clone();
                v[i] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn bubble_sort() {
        let mut v = [2, 1, 40, 30, 36, 12, 32, 18, 24, 45];
        super::bubble_sort(&mut v);
        assert_eq!(v, [1, 2, 12, 18, 24, 30, 32, 36, 40, 45])
    }
}
