/// ## 01 背包
/// https://time.geekbang.org/column/article/74788?screen=full

pub struct Knapsack {
    /// 物品的列表
    weight: Vec<usize>,
    /// 背包容量
    cap: usize,
    /// 可放入最大容量
    max: usize,
    /// 可放入最大容量
    len: usize,
    /// 记录
    men: Vec<Vec<bool>>,
    /// DP记录
    dp: Vec<Vec<bool>>,
    /// 递归深度
    depth: usize,
}

impl Knapsack {
    pub fn new(weight: Vec<usize>, cap: usize) -> Knapsack {
        let len = weight.len();
        Knapsack {
            weight,
            cap,
            max: 0,
            len,
            men: vec![vec![false; cap]; len],
            dp: vec![vec![false; cap + 1]; len],
            depth: 0,
        }
    }

    /// 这里的代码相当于在遍历一颗树
    /// ![](https://static001.geekbang.org/resource/image/42/ea/42ca6cec4ad034fc3e5c0605fbacecea.jpg?wh=1142*748)
    pub fn select(&mut self, i: usize, cw: usize, is_mem: bool) {
        self.depth += 1;

        if i == self.len || cw == self.cap {
            if cw > self.max {
                self.max = cw;
            }

            return;
        }

        // 因为树的遍历遍历是同步的，不是异步的。所以之前访问过的状态后续的访问都是重复的
        // 当递归树种的节点状态一致时，其后续的状态也是一致的。
        // 既，从同一个状态出发的递归树是一模一样的。说一后续不需要再次遍历
        if is_mem {
            if self.men[i][cw] {
                return;
            }
            self.men[i][cw] = true;
        }

        self.select(i + 1, cw, is_mem);
        if cw + self.weight[i] <= self.cap {
            self.select(i + 1, cw + self.weight[i], is_mem);
        }
    }

    pub fn dp_select_2(&mut self) -> usize {
        let mut dp = vec![false; self.cap + 1];

        dp[0] = true;
        if self.weight[0] <= self.cap {
            dp[self.weight[0]] = true;
        }

        for index in 1..self.len {
            let weight = self.weight[index];
            let surplus = self.cap as i32 - weight as i32;

            if surplus >= 0 {
                // 因为只使用了一维数组，状态是复用的，所以需要从后往前计算
                // 当从前往后计算时，相当于一个物品可能被使用多次。
                for j in (0..=surplus as usize).rev() {
                    if dp[j] {
                        dp[j + weight] = true;
                    }
                }
            }
        }

        println!("===============================");
        dp.iter().for_each(|it| {
            print!("{}\t", if *it { 1 } else { 0 })
        });
        println!("\n===============================");

        for index in (0..=self.cap).rev() {
            if dp[index] {
                return index;
            }
        }
        0
    }

    pub fn dp_select(&mut self) -> usize {
        // 处理第一个物品
        self.dp[0][0] = true;
        if self.weight[0] <= self.cap {
            self.dp[0][self.weight[0]] = true;
        }

        // 动态规划状态转移
        for i in 1..self.len {
            // 不把第i个物品放入背包
            // 当不把当前物品放入，相当于在之前的状态上加0，既复制上一个物品的状态即可
            for j in 0..=self.cap {
                if self.dp[i - 1][j] {
                    self.dp[i][j] = self.dp[i - 1][j];
                }
            }

            // 把第i个物品放入背包
            // 当把当前物品放入时，就是将之前状态的重量都加上当前重量
            // (self.cap - self.weight[i]) 的计算，是因为加入的重量不能超过包的容量
            if (self.cap as i32 - self.weight[i] as i32) > 0 {
                for j in 0..=(self.cap - self.weight[i]) {
                    if self.dp[i - 1][j] {
                        self.dp[i][j + self.weight[i]] = true;
                    }
                }
            }
        }

        let mut result = 0;
        for index in (0..=self.cap).rev() {
            if self.dp[self.len - 1][index] {
                result = index;
                break;
            }
        }

        println!("===============================");
        self.dp.iter()
            .for_each(|x| {
                x.iter()
                    .for_each(|y| {
                        print!("{}\t", if *y { 1 } else { 0 });
                    });
                println!();
            });
        println!("===============================");

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::bag_01::Knapsack;

    #[test]
    fn test_bag_01() {
        // 定义背包容量
        let capacity = 10;
        // 物品的数量既物品重量
        let obj_list = vec![2, 2, 4, 12, 6, 3, 1];

        let mut p01 = Knapsack::new(obj_list, capacity);

        p01.select(0, 0, true);
        println!("{}: {}=>{}", p01.max, p01.len, p01.depth);

        p01.select(0, 0, false);
        println!("{}: {}=>{}", p01.max, p01.len, p01.depth);

        let max = p01.dp_select();
        println!("DP Result =>{}", max);

        let max = p01.dp_select_2();
        println!("DP Result =>{}", max);
    }
}