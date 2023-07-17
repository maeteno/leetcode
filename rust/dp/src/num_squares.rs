use std::i32;

/// https://leetcode.cn/problems/perfect-squares/
/// 279. 完全平方数
/// 给你一个整数 n ，返回 和为 n 的完全平方数的最少数量 。
///
/// 完全平方数 是一个整数，其值等于另一个整数的平方；换句话说，其值等于一个整数自乘的积。例如，1、4、9 和 16 都是完全平方数，而 3 和 11 不是。
pub fn num_squares(n: i32) -> i32 {
    let mut list: Vec<i32> = vec![];
    for x in 1..=n {
        let i = x * x;
        if i == n {
            return 1;
        }

        if i > n {
            break;
        }

        list.push(i);
    }

    fn calc(list: &Vec<i32>, residue: i32, max: i32) -> i32 {
        if residue == 0 {
            return 0;
        }

        if residue < 0 {
            return max;
        }

        let mut min = max;
        for item in list {
            min = calc(list, residue - item, max).min(min);
        }

        return min + 1;
    }

    println!("{:?}", list);

    calc(&mut list, n, n)
}

pub fn num_squares_mem(n: i32) -> i32 {
    let mut list: Vec<i32> = vec![];
    for x in 1..=n {
        let i = x * x;
        if i == n {
            return 1;
        }

        if i > n {
            break;
        }

        list.push(i);
    }

    fn calc(list: &Vec<i32>, residue: i32, max: i32) -> i32 {
        if residue == 0 {
            return 0;
        }

        if residue < 0 {
            return max;
        }

        let mut min = max;
        for item in list {
            min = calc(list, residue - item, max).min(min);
        }

        return min + 1;
    }

    println!("{:?}", list);
    let mut _mem: Vec<i32> = vec![-1; n as usize];
    calc(&mut list, n, n)
}

// 测试用例
#[cfg(test)]
mod tests {
    use crate::num_squares::{num_squares, num_squares_mem};

    #[test]
    fn test_num_squares() {
        let result = num_squares(12);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_num_squares_1() {
        let result = num_squares(16);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_num_squares_mem() {
        let result = num_squares_mem(12);
        assert_eq!(result, 3);
    }
}