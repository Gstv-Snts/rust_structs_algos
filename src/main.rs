use std::fmt::Debug;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let v = [
        23, 5, 6, 12, 1, 56, 46, 21, 41, 4, 32, 61, 41, 6, 27, 3, 13, 53, 63,
    ];
    v.par_iter().for_each(|&i| println!("{}", i));
}
