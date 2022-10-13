//! 213. 打家劫舍 II
//! 你是一个专业的小偷，计划偷窃沿街的房屋，每间房内都藏有一定的现金。这个地方所有的房屋都 围成一圈 ，这意味着第一个房屋和最后一个房屋是紧挨着的。同时，相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警 。
//! 给定一个代表每个房屋存放金额的非负整数数组，计算你 在不触动警报装置的情况下 ，今晚能够偷窃到的最高金额。

pub fn rob(nums: Vec<i32>) -> i32 {
    let len = nums.len();

    if len == 1 {
        return nums[0];
    }

    if len == 2 {
        return std::cmp::max(nums[0], nums[1]);
    }

    // 0: 从第一个房间开始偷，最后一个不能偷
    // 1: 从第二个房间开始偷，偷到最后一个
    let mut mem = vec![vec![-1; len]; 2];

    mem[0][0] = nums[0];
    mem[1][1] = nums[1];

    for i in 1..len {
        // 从第一个开始
        if i < len - 1 {
            if i == 1 {
                mem[0][1] = std::cmp::max(nums[0], nums[1]);
            } else {
                // 第n个房间的金额
                mem[0][i] = std::cmp::max(mem[0][i - 1], mem[0][i - 2] + nums[i]);
            }
        }

        // 从第二个开始
        if i > 1 {
            if i == 2 {
                mem[1][2] = std::cmp::max(nums[1], nums[2]);
            } else {
                // 第n个房间的金额
                mem[1][i] = std::cmp::max(mem[1][i - 1], mem[1][i - 2] + nums[i]);
            }
        }
    }

    std::cmp::max(mem[0][len - 2], mem[1][len - 1])
}


#[cfg(test)]
mod tests {
    use crate::no_213_medium::rob;

    #[test]
    fn test_rob() {
        let nums = vec![2, 3, 4, 5, 6];
        let max = rob(nums);

        println!("{}", max);
    }

    #[test]
    fn test_rob_2() {
        let nums = vec![2, 3, 4];
        let max = rob(nums);

        println!("{}", max);
    }

    #[test]
    fn test_rob_3() {
        let nums = vec![2, 3];
        let max = rob(nums);

        println!("{}", max);
    }
}
