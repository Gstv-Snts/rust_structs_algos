pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    for i in 0..nums.len() - 1 {
        if i + 1 < nums.len() {
            if nums[i].eq(&nums[i + 1]) {
                nums.remove(i);
            }
        }
    }
    (nums.len() - 1) as i32
}
