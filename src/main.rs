use rust_structs_algos::algos::sort::quick_sort;

fn main() {
    let mut v = [1, 2, 6, 5, 8, 3, 11, 5, 7, 10, 23, 40, 4, 1, 3];
    println!("{:?}", v);
    quick_sort(&mut v);
    println!("{:?}", v);
}
