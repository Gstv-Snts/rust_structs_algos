use rust_structs_algos::structs::graph::AdjacentList;

#[tokio::main]
async fn main() {
    let mut al: AdjacentList<usize> = AdjacentList::new();
    al.insert(0);
    al.insert(1);
    al.insert(2);
    al.insert(3);
    al.insert(4);
    al.insert(5);
    al.insert(6);
    al.insert(7);
    al.connect(0, 1);
    al.connect(0, 2);
    al.connect(0, 3);
    al.connect(1, 4);
    al.connect(1, 5);
    al.connect(2, 6);
    al.connect(3, 7);
    for i in &al.0 {
        println!("{:?}", i);
    }
    al.dfs(0);
}
