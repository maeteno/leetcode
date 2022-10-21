/// 01 背包的升级版本，携带价值最大的商品
///
pub struct Goods {
    weight: i32,
    value: i32,
}

impl Goods {
    pub fn new(weight: i32, value: i32) -> Goods {
        Goods { weight, value }
    }
}

pub struct ValuableKnapsack {
    /// 物品的列表
    goods: Vec<Goods>,
    /// 背包容量
    cap: i32,
}

impl ValuableKnapsack {
    pub fn select(&self) -> i32 {
        let len = self.goods.len();
        let mut dp = vec![-1; self.cap as usize + 1];
        let Goods { weight, value } = self.goods[0];

        // 此处表示一个商品都没有放，背包为空
        dp[0] = 0;
        // 初始化第一个商品，后续以第一个商品未初始状态开始计算
        if weight <= self.cap {
            dp[weight as usize] = value;
        }

        for x in 1..len {
            let Goods { weight, value } = self.goods[x];
            let surplus = (self.cap - weight) as i32;
            if surplus >= 0 {
                for y in (0..=surplus as usize).rev() {
                    if dp[y] >= 0 && dp[y + weight as usize] < dp[y] + value {
                        dp[y + weight as usize] = dp[y] + value;
                    }
                }
            }
        }

        dp.iter().for_each(|v| {
            print!("{}\t", v);
        });
        println!();

        match dp.iter().max() {
            Some(v) => *v,
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bag_01_2::{Goods, ValuableKnapsack};
    #[test]
    pub fn test_dp_select() {
        let goods = vec![
            Goods::new(2, 200),
            Goods::new(3, 50),
            Goods::new(3, 80),
            Goods::new(3, 70),
        ];

        let knapsack = ValuableKnapsack { goods, cap: 9 };

        let value = knapsack.select();

        println!("max Value: {}", value);
    }
}
