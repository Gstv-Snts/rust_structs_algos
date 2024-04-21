pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if (nums[i] + nums[j]) == target {
                return vec![i.try_into().unwrap(), j.try_into().unwrap()];
            }
        }
    }
    vec![]
}

mod test {
    use crate::algos::leetcode::two_sum;

    #[test]
    fn two_sum_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }
    #[test]
    fn two_sum_2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2])
    }
    #[test]
    fn two_sum_3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1])
    }
    #[test]
    fn two_sum_4() {
        assert_eq!(two_sum(vec![2, 5, 5, 11], 10), vec![1, 2])
    }
}
