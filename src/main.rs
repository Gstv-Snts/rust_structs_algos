use rust_structs_algos::structs::tree::Heap;

#[tokio::main]
async fn main() {
    let mut h = Heap::new();
    h.insert(10);
}
