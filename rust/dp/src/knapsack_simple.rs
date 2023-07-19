// 可放入背包的最大重量
pub fn bag_dfs(weight: i32, list: Vec<i32>) -> i32 {
    fn calc(weight: i32, list: &Vec<i32>, index: usize, cur_weight: i32, take: bool) -> i32 {
        let mut new_weight = cur_weight;

        if take {
            new_weight = cur_weight + list[index];

            if new_weight > weight {
                return cur_weight;
            }

            if new_weight == weight {
                return new_weight;
            }
        }

        let next = index + 1;

        if next == list.len() {
            return new_weight;
        }

        calc(weight, &list, next, new_weight, false).max(calc(weight, &list, next, new_weight, true))
    }

    calc(weight, &list, 0, 0, false).max(calc(weight, &list, 0, 0, true))
}

// 测试用例
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bag_dfs() {
        let weight = 12;
        let list = vec![3, 4, 5, 6];
        let result = bag_dfs(weight, list);
        assert_eq!(result, 12);
    }
}