/// https://leetcode.cn/problems/perfect-squares/
/// 279. 完全平方数
/// 给你一个整数 n ，返回 和为 n 的完全平方数的最少数量 。
///
/// 完全平方数 是一个整数，其值等于另一个整数的平方；换句话说，其值等于一个整数自乘的积。例如，1、4、9 和 16 都是完全平方数，而 3 和 11 不是。
pub fn num_squares(n: i32) -> i32 {
    let mut list = match get_x(n) {
        Ok(value) => value,
        Err(value) => return value,
    };

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

    fn calc(list: &Vec<i32>, residue: i32, mem: &mut Vec<i32>) -> i32 {
        if residue == 0 {
            return 0;
        }

        if residue < 0 {
            return -1;
        }

        if mem[residue as usize] == -2 {
            return -1;
        }

        if mem[residue as usize] != -1 {
            return mem[residue as usize];
        }

        let mut min = -1;
        for item in list {
            let c_result = calc(list, residue - item, mem);
            if c_result != -1 {
                if min == -1 {
                    min = c_result;
                } else {
                    min = c_result.min(min);
                }
            }
        }

        if min == -1 {
            mem[residue as usize] = -2;
            return -1;
        }

        mem[residue as usize] = min + 1;
        return min + 1;
    }

    println!("{:?}", list);
    let mut mem: Vec<i32> = vec![-1; n as usize + 1];
    calc(&mut list, n, &mut mem)
}

pub fn num_squares_dp(n: i32) -> i32 {
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

    let mut mem: Vec<i32> = vec![-1; n as usize + 1];
    mem[0] = 0;
    mem[1] = 1;
    mem[2] = 2;

    if n > 2 {
        for x in 3..=n {
            if list.contains(&x) {
                mem[x as usize] = 1;
                continue;
            }

            let mut min = x;
            for y in 1..x {
                min = min.min(mem[y as usize] + mem[(x - y) as usize]);
            }

            mem[x as usize] = min;
        }
    }

    mem[n as usize]
}

pub fn num_squares_dp_2(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    let mut mem: Vec<i32> = vec![-1; n as usize + 1];
    for x in 1..=n {
        let i = x * x;

        if i > n {
            break;
        }

        mem[i as usize] = 1;
    }

    for x in 2..=n {
        if mem[x as usize] == 1 {
            continue;
        }
        let mut min = x;

        for y in 1..x {
            min = min.min(mem[y as usize] + mem[(x - y) as usize]);
            if min == 2 {
                continue;
            }
        }
        mem[x as usize] = min;
    }

    mem[n as usize]
}

fn get_x(n: i32) -> Result<Vec<i32>, i32> {
    let mut list: Vec<i32> = vec![];
    for x in 1..=n {
        let i = x * x;
        if i == n {
            return Err(1);
        }

        if i > n {
            break;
        }

        list.push(i);
    }
    Ok(list)
}

// 测试用例
#[cfg(test)]
mod tests {
    use crate::num_squares::{num_squares, num_squares_dp, num_squares_dp_2, num_squares_mem};

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

    #[test]
    fn test_num_squares_dp() {
        let result = num_squares_dp(12);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_num_squares_dp_2() {
        let result = num_squares_dp_2(1900);
        assert_eq!(result, 3);
    }
}