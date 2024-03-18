use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub mod algos;
pub mod structs;

pub fn regular_iter(a: &Vec<u32>) -> u32 {
    a.iter().map(|&num| num * num).sum()
}
pub fn rayon_par_iter(a: &Vec<u32>) -> u32 {
    a.par_iter().map(|&num| num * num).sum()
}
