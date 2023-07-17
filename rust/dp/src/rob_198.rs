/// https://leetcode.cn/problems/house-robber/description/
/// 198. 打家劫舍
///  你是一个专业的小偷，计划偷窃沿街的房屋。每间房内都藏有一定的现金，影响你偷窃的唯一制约因素就是相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警。
///
///  给定一个代表每个房屋存放金额的非负整数数组，计算你 不触动警报装置的情况下 ，一夜之内能够偷窃到的最高金额。
pub fn rob(nums: Vec<i32>) -> i32 {
    fn calc(nums: &Vec<i32>, index: i32) -> i32 {
        if index < 0 {
            return 0;
        }

        let mut max = 0;

        if index - 2 >= 0 {
            for x in (0..=index - 2).rev() {
                max = calc(nums, x).max(max);
            }
        }

        nums[index as usize] + max
    }

    calc(&nums, nums.len() as i32 - 1).max(calc(&nums, nums.len() as i32 - 2))
}

pub fn rob_mem(nums: Vec<i32>) -> i32 {
    fn calc(nums: &Vec<i32>, index: i32, mem: &mut Vec<i32>) -> i32 {
        if index < 0 {
            return 0;
        }

        if mem[index as usize] != -1 {
            return mem[index as usize];
        }

        let mut max = 0;
        if index - 2 >= 0 {
            for x in (0..=index - 2).rev() {
                max = calc(nums, x, mem).max(max);
            }
        }

        mem[index as usize] = nums[index as usize] + max;
        mem[index as usize]
    }

    let mut mem = vec![-1; nums.len()];
    calc(&nums, nums.len() as i32 - 1, &mut mem).max(calc(&nums, nums.len() as i32 - 2, &mut mem))
}

pub fn rob_dp(nums: Vec<i32>) -> i32 {
    let len = nums.len();

    if len == 1 {
        return nums[0];
    }

    let mut dp = vec![-1; nums.len()];
    dp[0] = nums[0];
    dp[1] = nums[1];

    for x in 2..len {
        let mut max = 0;

        for y in 0..x - 1 {
            max = dp[y].max(max)
        }

        dp[x] = nums[x] + max;
    }

    dp[len - 1].max(dp[len - 2])
}

// 测试用例
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(rob(vec![2, 1, 1, 2]), 4);
    }

    #[test]
    fn test_rob_2() {
        assert_eq!(rob(vec![2, 1, 1, 2]), 4);
    }

    #[test]
    fn test_rob_mem() {
        assert_eq!(rob_mem(vec![1, 2, 3, 1]), 4);
        assert_eq!(rob_mem(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(rob_mem(vec![2, 1, 1, 2]), 4);
        assert_eq!(rob_mem(vec![114, 117, 207, 117, 235, 82, 90, 67, 143, 146, 53, 108, 200, 91, 80, 223, 58, 170, 110, 236, 81, 90, 222, 160, 165, 195, 187, 199, 114, 235, 197, 187, 69, 129, 64, 214, 228, 78, 188, 67, 205, 94, 205, 169, 241, 202, 144, 240]), 4173);
    }

    #[test]
    fn test_rob_dp() {
        assert_eq!(rob_dp(vec![1, 2, 3, 1]), 4);
        assert_eq!(rob_dp(vec![2, 7, 9, 3, 1]), 12);
        assert_eq!(rob_dp(vec![2, 1, 1, 2]), 4);
        assert_eq!(rob_dp(vec![114, 117, 207, 117, 235, 82, 90, 67, 143, 146, 53, 108, 200, 91, 80, 223, 58, 170, 110, 236, 81, 90, 222, 160, 165, 195, 187, 199, 114, 235, 197, 187, 69, 129, 64, 214, 228, 78, 188, 67, 205, 94, 205, 169, 241, 202, 144, 240]), 4173);
    }
}