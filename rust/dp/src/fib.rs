// 回溯法的斐波那契数列
pub fn fib(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    if n == 1 || n == 2 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}

// 记忆化存储版本，递归
pub fn fib_mem(num: i32) -> i32 {
    let count = num as usize + 1;
    let mut mem: Vec<i32> = vec![-1; count];
    mem[0] = 0;
    mem[1] = 1;
    mem[2] = 1;

    fn fib2(men: &mut Vec<i32>, num: i32) -> i32 {
        if num < 1 {
            return 0;
        }

        if men[num as usize] != -1 {
            return men[num as usize];
        }

        men[num as usize] = fib2(men, num - 1) + fib2(men, num - 2);

        men[num as usize]
    }

    fib2(&mut mem, num)
}

// 动态规划版本,递推
pub fn fib_dp(num: i64) -> i64 {
    let count = num as usize + 1;
    let mut mem: Vec<i64> = vec![-1; count];
    mem[0] = 0;
    mem[1] = 1;
    mem[2] = 1;

    for index in 3..count {
        mem[index] = mem[index - 1] + mem[index - 2];
    }

    mem[count - 1]
}


#[cfg(test)]
mod tests {
    use crate::fib::{fib, fib_dp, fib_mem};

    #[test]
    fn test_fib() {
        // 计算计时
        let start = std::time::Instant::now();
        println!("{}", fib(40));
        let end = std::time::Instant::now();
        println!("{}us", (end - start).as_micros());

        let start = std::time::Instant::now();
        println!("{}", fib_mem(40));
        let end = std::time::Instant::now();
        println!("{}us", (end - start).as_micros());

        let start = std::time::Instant::now();
        println!("{}", fib_dp(40));
        let end = std::time::Instant::now();
        println!("{}us", (end - start).as_micros());
    }
}

