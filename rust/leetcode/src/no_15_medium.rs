/// 15. 三数之和
/// 给你一个整数数组 nums ，判断是否存在三元组 [nums[i], nums[j], nums[k]] 满足 i != j、i != k 且 j != k ，同时还满足 nums[i] + nums[j] + nums[k] == 0 。请你返回所有和为 0 且不重复的三元组。
/// 注意：答案中不可以包含重复的三元组。
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    println!("{:?}", nums);

    for ele in nums {
        println!("{}", ele)
    }

    vec![vec![], vec![]]
}

#[cfg(test)]
mod tests {
    use super::three_sum;

    #[test]
    fn test_three_sum() {
        let result = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(result.len(), 2);
    }
}
