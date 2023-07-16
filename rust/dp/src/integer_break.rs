pub fn integer_break(n: i32) -> i32 {
    if n == 2 {
        return n;
    }

    let mut max = n;
    // n/2 排除了一半的计算，a*b=b*a
    for m in 2..=n / 2 {
        let tmp = integer_break(m) * integer_break(n - m);

        if tmp > max {
            max = tmp
        }
    }

    max
}

pub fn integer_break_mem(n: i32) -> i32 {
    if n == 2 {
        return 1;
    }

    if n == 3 {
        return 2;
    }

    fn calc(n: i32, mem: &mut Vec<i32>) -> i32 {
        if n == 2 {
            return n;
        }

        if mem[n as usize] != -1 {
            return mem[n as usize];
        }

        let mut max = n;
        // n/2 排除了一半的计算，a*b=b*a
        for m in 1..=n / 2 {
            let tmp = calc(m, mem) * calc(n - m, mem);

            if tmp > max {
                max = tmp
            }
        }

        mem[n as usize] = max;
        max
    }

    let mut mem = vec![-1; n as usize + 1];
    calc(n, &mut mem)
}

pub fn integer_break_dp(n: i32) -> i32 {
    if n == 2 {
        return 1;
    }

    if n == 3 {
        return 2;
    }

    let mut dp = vec![-1; n as usize + 1];
    dp[1] = 1;
    dp[2] = 2;
    dp[3] = 3;

    for index in 4..=n {
        let mut max = index;

        // n/2 排除了一半的计算，a*b=b*a
        for m in 2..=index {
            let tmp = dp[m as usize] * dp[(index - m) as usize];

            if tmp > max {
                max = tmp
            }
        }

        dp[index as usize] = max;
    }

    dp[n as usize]
}

#[cfg(test)]
mod tests {
    use crate::integer_break::{integer_break, integer_break_dp, integer_break_mem};

    #[test]
    fn test_integer_break() {
        let r = integer_break(10);
        assert_eq!(r, 36)
    }

    #[test]
    fn test_integer_break_mem() {
        let r = integer_break_mem(10);
        assert_eq!(r, 36)
    }

    #[test]
    fn test_integer_break_mem_2() {
        let r = integer_break_mem(2);
        assert_eq!(r, 1)
    }

    #[test]
    fn test_integer_break_mem_3() {
        let r = integer_break_mem(3);
        assert_eq!(r, 2)
    }

    #[test]
    fn test_integer_break_dp() {
        let r = integer_break_dp(10);
        assert_eq!(r, 36)
    }

    #[test]
    fn test_integer_break_dp_2() {
        let r = integer_break_dp(4);
        assert_eq!(r, 4)
    }

    #[test]
    fn test_integer_break_dp_3() {
        let r = integer_break_dp(3);
        assert_eq!(r, 2)
    }
}