pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    // f(row,col) = f(row,col) + min(f(row,col),f(row,col));

    fn mini(a: i32, b: i32) -> i32 {
        if a > b { b } else { a }
    }

    fn calc(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
        if row == 0 && col == 0 {
            grid[0][0]
        } else if col == 0 {
            grid[col][row] + calc(grid, row - 1, col)
        } else if row == 0 {
            grid[col][row] + calc(grid, row, col - 1)
        } else {
            grid[col][row] + mini(calc(grid, row - 1, col), calc(grid, row, col - 1))
        }
    }

    let row = grid[0].len();
    let col = grid.len();

    calc(&grid, row - 1, col - 1)
}

pub fn min_path_sum_mem(grid: Vec<Vec<i32>>) -> i32 {
    // f(row,col) = f(row,col) + min(f(row -1,col),f(row,col -1));

    fn mini(a: i32, b: i32) -> i32 {
        if a > b { b } else { a }
    }

    fn calc(grid: &Vec<Vec<i32>>, row: usize, col: usize, mem: &mut Vec<Vec<i32>>) -> i32 {
        if mem[col][row] != -1 {
            return mem[col][row];
        }

        if row == 0 && col == 0 {
            return grid[0][0];
        }

        mem[col][row] = if col == 0 {
            grid[col][row] + calc(grid, row - 1, col, mem)
        } else if row == 0 {
            grid[col][row] + calc(grid, row, col - 1, mem)
        } else {
            grid[col][row] + mini(calc(grid, row - 1, col, mem), calc(grid, row, col - 1, mem))
        };

        mem[col][row]
    }

    let row = grid[0].len();
    let col = grid.len();

    let mut mem = vec![vec![-1; row]; col];

    calc(&grid, row - 1, col - 1, &mut mem)
}

// 动态规划
pub fn min_path_sum_dp(grid: Vec<Vec<i32>>) -> i32 {
    // f(row,col) = f(row,col) + min(f(row -1,col),f(row,col -1));

    let row = grid[0].len();
    let col = grid.len();

    let mut dp = vec![vec![-1; row]; col];

    for x in 0..col {
        for y in 0..row {
            dp[x][y] = if x == 0 && y == 0 {
                grid[0][0]
            } else if x == 0 {
                grid[x][y] + dp[x][y - 1]
            } else if y == 0 {
                grid[x][y] + dp[x - 1][y]
            } else {
                grid[x][y] + dp[x - 1][y].min(dp[x][y - 1])
            };
        }
    }

    dp[col - 1][row - 1]
}


// 测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_total() {
        let grid = vec![
            vec![1, 3, 1],
            vec![1, 5, 1],
            vec![4, 2, 1],
        ];

        assert_eq!(min_path_sum(grid), 7);
    }

    #[test]
    fn test_minimum_total_1() {
        let grid = vec![
            vec![1, 1, 3],
            vec![1, 5, 1],
        ];

        assert_eq!(min_path_sum(grid), 6);
    }

    #[test]
    fn test_minimum_total_mem() {
        let grid = vec![
            vec![1, 3, 1],
            vec![1, 5, 1],
            vec![4, 2, 1],
        ];

        assert_eq!(min_path_sum_mem(grid), 7);
    }

    #[test]
    fn test_minimum_total_dp() {
        let grid = vec![
            vec![1, 3, 1],
            vec![1, 5, 1],
            vec![4, 2, 1],
        ];

        assert_eq!(min_path_sum_dp(grid), 7);
    }
}