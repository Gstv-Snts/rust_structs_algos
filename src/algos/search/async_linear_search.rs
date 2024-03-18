use async_recursion::async_recursion;
pub async fn async_linear_search<T: Ord + Clone + Send>(vec: &Vec<T>, target: T) -> Option<T> {
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
