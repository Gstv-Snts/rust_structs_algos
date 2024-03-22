use rayon::prelude::*;
pub mod algos;
pub mod structs;
pub fn sum_of_squares(input: &[i32]) -> i32 {
    input
        .iter() // <-- just change that!
        .map(|&i| i * i)
        .sum()
}
pub fn rayon_sum_of_squares(input: &[i32]) -> i32 {
    input
        .par_iter() // <-- just change that!
        .map(|&i| i * i)
        .sum()
}
