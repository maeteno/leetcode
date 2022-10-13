//! 746. 使用最小花费爬楼梯
//! 给你一个整数数组 cost ，其中 cost[i] 是从楼梯第 i 个台阶向上爬需要支付的费用。一旦你支付此费用，即可选择向上爬一个或者两个台阶。
//
// 你可以选择从下标为 0 或下标为 1 的台阶开始爬楼梯。
//
// 请你计算并返回达到楼梯顶部的最低花费

use std::cmp::min;

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut mem = vec![0; cost.len()];
    mem[0] = cost[0];
    mem[1] = cost[1];

    for index in 2..cost.len() {
        mem[index] = min(mem[index - 1], mem[index - 2]) + cost[index];
    }

    min(mem[cost.len() - 1], mem[cost.len() - 2])
}

pub fn min_cost_climbing_stairs_2(cost: Vec<i32>) -> i32 {
    let len = cost.len();
    let mut mem = vec![0; len + 1];

    for index in 2..=len {
        mem[index] = min(mem[index - 1] + cost[index - 1], mem[index - 2] + cost[index - 2]);
    }

    mem[len]
}

#[cfg(test)]
mod tests {
    use crate::no_746_easy::{min_cost_climbing_stairs, min_cost_climbing_stairs_2};

    #[test]
    fn test_min_cost_climbing_stairs() {
        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let min = min_cost_climbing_stairs(cost);

        println!("{}", min);
    }

    #[test]
    fn test_min_cost_climbing_stairs_2() {
        let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let min = min_cost_climbing_stairs_2(cost);

        println!("{}", min);
    }
}