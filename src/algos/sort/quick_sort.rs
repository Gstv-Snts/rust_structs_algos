use std::{fmt::Debug, usize};

pub fn quick_sort<T: Sized + Ord + Clone + Debug + Copy + Send, const COUNT: usize>(
    arr: &mut [T; COUNT],
) {
    recurse(&mut arr[..])
}
pub fn recurse<T: Sized + Ord + Clone + Debug + Copy + Send>(v: &mut [T]) {
    if v.len() < 2 {
        return;
    }
    let mut h = 0;
    let mut t = v.len() - 1;
    let mut p = v.len() / 2;
    let tmp = v[p].clone();
    v[p] = v[t].clone();
    v[t] = tmp;
    p = t;
    t -= 1;
    for _ in 0..v.len() {
        if h == p {
            break;
        }
        if v[t] > v[p] {
            let tmp = v[t].clone();
            v[t] = v[p].clone();
            v[p] = tmp;
            p -= 1;
            if t > 0 {
                t -= 1;
            }
        } else {
            let tmp = v[h].clone();
            v[h] = v[t].clone();
            v[t] = tmp;
            h += 1;
        }
    }
    recurse(&mut v[..p]);
    recurse(&mut v[p..]);
}
#[cfg(test)]
mod test {
    #[test]
    fn quick_sort() {
        let mut arr = [10, 20, 40, 2, 55, 14, 16, 19, 24];
        super::quick_sort(&mut arr);
        assert_eq!(arr, [2, 10, 14, 16, 19, 20, 24, 40, 55]);
    }
}
