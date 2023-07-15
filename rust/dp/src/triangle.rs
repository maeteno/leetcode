// 递归方案
pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    fn mini(a: i32, b: i32) -> i32 {
        if a > b { b } else { a }
    }

    fn backtrack(triangle: &Vec<Vec<i32>>, row: usize, column: usize) -> i32 {
        let now = triangle[row][column];

        if row == triangle.len() - 1 {
            return now;
        }

        return now + mini(backtrack(triangle, row + 1, column), backtrack(triangle, row + 1, column + 1));
    }

    backtrack(&triangle, 0, 0)
}

// 递归方案 记忆化存储
pub fn minimum_total_mem(triangle: Vec<Vec<i32>>) -> i32 {
    fn mini(a: i32, b: i32) -> i32 {
        if a > b { b } else { a }
    }

    fn backtrack(triangle: &Vec<Vec<i32>>, row: usize, column: usize, mem: &mut Vec<Vec<i32>>) -> i32 {
        if mem[row][column] == -1 {
            let now = triangle[row][column];

            // 递归终止
            if row == triangle.len() - 1 {
                return now;
            }

            let sum = now + mini(backtrack(triangle, row + 1, column, mem), backtrack(triangle, row + 1, column + 1, mem));
            mem[row][column] = sum;
        }

        return mem[row][column];
    }
    let mut mem = vec![vec![-1; triangle[triangle.len() - 1].len()]; triangle.len()];
    backtrack(&triangle, 0, 0, &mut mem)
}

// 动态规划
pub fn minimum_total_dp(triangle: Vec<Vec<i32>>) -> i32 {
    fn mini(a: i32, b: i32) -> i32 {
        if a > b { b } else { a }
    }
    let col = triangle.len();
    let row = triangle[col - 1].len();

    let mut dp = vec![vec![-1; row]; col];

    // 设置初始值
    for index in 0..row {
        dp[col - 1][index] = triangle[col - 1][index]
    }

    let mut x = col - 1;

    while x > 0 {
        for y in 0..x {
            dp[x - 1][y] = triangle[x - 1][y] + mini(dp[x][y], dp[x][y + 1]);
        }

        x -= 1;
    }

    dp[0][0]
}

#[cfg(test)]
mod tests {
    use crate::triangle::{minimum_total, minimum_total_dp, minimum_total_mem};

    #[test]
    fn test_minimum_total() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        let result = minimum_total(triangle);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_minimum_total_mem() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        let result = minimum_total_mem(triangle);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_minimum_total_dp() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        let result = minimum_total_dp(triangle);
        assert_eq!(result, 11);
    }
}