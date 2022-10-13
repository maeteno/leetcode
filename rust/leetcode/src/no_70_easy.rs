//! 70. 爬楼梯
//! 假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
//
// 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？


pub fn climb_stairs(n: i32) -> i32 {
    let mut mem = vec![0; n as usize + 1];
    mem[0] = 1;
    mem[1] = 1;

    for index in 2..=n as usize {
        mem[index] = mem[index - 1] + mem[index - 2];
    }

    mem[n as usize]
}

pub fn climb_stairs2(n: i32) -> i32 {
    let mut mem = vec![1, 1, 1];

    for _ in 2..=n as usize {
        mem[2] = mem[1] + mem[0];
        mem[0] = mem[1];
        mem[1] = mem[2];
    }

    mem[2]
}

pub struct Stairs {
    pub mem: Vec<i32>,
}

impl Stairs {
    pub fn new(n: i32) -> Stairs {
        let mut mem: Vec<i32> = vec![0; n as usize + 1];
        mem[0] = 1;
        mem[1] = 1;
        return Stairs {
            mem
        };
    }

    pub fn climb(&mut self, n: usize) -> i32 {
        if self.mem[n] != 0 {
            return self.mem[n];
        }
        self.mem[n] = self.climb(n - 1) + self.climb(n - 2);

        return self.mem[n];
    }
}

// 你好
#[cfg(test)]
mod tests {
    use crate::no_70_easy::{climb_stairs, climb_stairs2, Stairs};

    #[test]
    fn test_climb_stairs() {
        let count = climb_stairs(45);
        println!("count: {}", count)
    }

    #[test]
    fn test_climb_stairs2() {
        let count = climb_stairs2(45);
        println!("count: {}", count)
    }

    #[test]
    pub fn test_climb_stairs3() {
        let mut stairs = Stairs::new(45);
        let count = stairs.climb(45);
        println!("count: {}", count)
    }
}
