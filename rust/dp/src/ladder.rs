pub fn backtrack(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }

    if num == 1 {
        return 1;
    }

    if num == 2 {
        return 2;
    }

    backtrack(num - 1) + backtrack(num - 2)
}


pub fn backtrack_mem(num: i32) -> i32 {
    fn calc(mem: &mut Vec<i32>, num: i32) -> i32 {
        let index = num as usize;

        if mem[num as usize] != -1 {
            return mem[index];
        }

        mem[index] = calc(mem, num - 1) + calc(mem, num - 2);

        mem[index]
    }

    let mut mem = vec![-1; num as usize + 1];
    mem[0] = 0;
    mem[1] = 1;
    mem[2] = 2;

    calc(&mut mem, num)
}

pub fn ladder_dp(num: i32) -> i32 {
    let mut dp = vec![-1; num as usize + 1];

    dp[0] = 0;
    dp[1] = 1;
    dp[2] = 2;

    if num <= 2 {
        return dp[num as usize];
    }

    for index in 3..=num as usize {
        dp[index] = dp[index - 1] + dp[index - 2];
    }


    dp[num as usize]
}

#[cfg(test)]
mod test {
    use crate::ladder::{backtrack, backtrack_mem, ladder_dp};

    #[test]
    fn test() {
        let start = std::time::Instant::now();
        println!("{}", backtrack(40));
        let end = std::time::Instant::now();
        println!("{}us", (end - start).as_micros());

        println!();
        let start = std::time::Instant::now();
        println!("{}", backtrack_mem(40));
        let end = std::time::Instant::now();
        println!("{}us", (end - start).as_micros());

        println!();
        let start = std::time::Instant::now();
        println!("{}", ladder_dp(40));
        let end = std::time::Instant::now();
        println!("{}us", (end - start).as_micros());
    }
}
