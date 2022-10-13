//! 198. 打家劫舍
//! 你是一个专业的小偷，计划偷窃沿街的房屋。每间房内都藏有一定的现金，影响你偷窃的唯一制约因素就是相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警。
//! 给定一个代表每个房屋存放金额的非负整数数组，计算你 不触动警报装置的情况下 ，一夜之内能够偷窃到的最高金额。
//!
use std::cmp::max;

pub fn rob(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut mem = vec![0; len];
    mem[0] = nums[0];

    for i in 1..len {
        if i == 1 {
            mem[1] = std::cmp::max(nums[0], nums[1]);
        } else {
            // 第n个房间的金额
            mem[i] = std::cmp::max(mem[i - 1], mem[i - 2] + nums[i]);
        }
    }

    mem[len - 1]
}

pub fn rob_2(nums: &Vec<i32>, i: usize) -> i32 {
    println!("Index: {}", i);

    if i == 0 {
        return nums[0];
    }

    if i == 1 {
        return max(nums[0], nums[1]);
    }

    let max_num = max(rob_2(&nums, i - 1), rob_2(&nums, i - 2) + nums[i]);

    return max_num;
}

pub fn rob_dp(nums: &Vec<i32>, i: usize, mem: &mut Vec<i32>) -> i32 {
    if mem[i] != 0 {
        return mem[i];
    }

    println!("Index: {}", i);
    if i == 0 {
        return nums[0];
    }

    if i == 1 {
        return max(nums[0], nums[1]);
    }

    let max_num = max(rob_dp(&nums, i - 1, mem), rob_dp(&nums, i - 2, mem) + nums[i]);

    mem[i] = max_num;

    return max_num;
}


#[cfg(test)]
mod tests {
    use crate::no_198_medium::{rob, rob_2, rob_dp};

    #[test]
    fn test_rob() {
        let nums = vec![3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9];

        let max = rob(nums);

        assert_eq!(max, 111);
        println!("{}", max);
    }

    #[test]
    fn test_rob_2() {
        let nums = vec![3, 2, 3, 9, 3, 2, 3, 9, 3];

        let max = rob_2(&nums, nums.len() - 1);

        assert_eq!(max, 23);
        println!("{}", max);
    }

    #[test]
    fn test_rob_dp() {
        let nums = vec![3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9, 3, 2, 3, 9];
        let mut mem = vec![0; nums.len()];

        let max = rob_dp(&nums, nums.len() - 1, &mut mem);

        assert_eq!(max, 111);
        println!("{}", max);
    }
}