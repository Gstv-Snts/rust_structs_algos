use async_recursion::async_recursion;
use std::{fmt::Debug, usize};

pub async fn async_quick_sort<T: Sized + Ord + Clone + Debug + Copy + Send, const COUNT: usize>(
    arr: &mut [T; COUNT],
) {
    recurse(&mut arr[..]).await;
}

#[async_recursion]
async fn recurse<T: Sized + Ord + Clone + Debug + Copy + Send>(v: &mut [T]) {
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
    recurse(&mut v[..p]).await;
    recurse(&mut v[p..]).await;
}
