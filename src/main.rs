use async_recursion::async_recursion;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 6, 5, 8, 3, 11, 5, 7, 10, 23, 40, 4, 1, 3];
    println!("{:?}", linear_search(&v, 23).await)
}

async fn linear_search<T: Ord + Clone>(vec: &Vec<T>, target: T) -> Option<T> {
    recurse(vec, target, 0).await
}

#[async_recursion(?Send)]
async fn recurse<T: Ord + Clone>(vec: &Vec<T>, target: T, i: usize) -> Option<T> {
    if i >= vec.len() {
        return None;
    }
    if vec[i] == target {
        return Some(vec[i].clone());
    }
    recurse(vec, target, i + 1).await
}
