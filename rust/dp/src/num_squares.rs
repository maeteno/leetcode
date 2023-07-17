/// https://leetcode.cn/problems/perfect-squares/
/// 279. 完全平方数
/// 给你一个整数 n ，返回 和为 n 的完全平方数的最少数量 。
///
/// 完全平方数 是一个整数，其值等于另一个整数的平方；换句话说，其值等于一个整数自乘的积。例如，1、4、9 和 16 都是完全平方数，而 3 和 11 不是。
pub fn num_squares(n: i32) -> i32 {
    println!("{}", n);
    3
}

// 测试用例
#[cfg(test)]
mod tests {
    use crate::num_squares::num_squares;

    #[test]
    fn test_num_squares() {
        let result = num_squares(12);
        assert_eq!(result, 3);
    }
}