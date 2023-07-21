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

pub fn bag_dfs_1(list: Vec<i32>, weight: i32) -> i32 {
    fn calc(list: &Vec<i32>, weight: i32) -> i32 {
        let mut max = 0;
        for index in 0..list.len() {
            let item = list[index];
            if item < weight {
                let mut residue = vec![];
                list.clone_into(&mut residue);
                residue.remove(index);

                let size = item + calc(&residue, weight - item);

                if size <= weight {
                    max = max.max(size)
                }
            }

            if item == weight {
                max = weight;
                break;
            }
        }
        max
    }

    calc(&list, weight)
}

pub fn bag_mem(list: Vec<i32>, weight: i32) -> i32 {
    fn calc(list: &Vec<i32>, weight: i32, mem: &mut Vec<i32>) -> i32 {
        if mem[weight as usize] != -1 {
            return mem[weight as usize];
        }

        let mut max = 0;
        for index in 0..list.len() {
            let item = list[index];
            if item < weight {
                let mut residue = vec![];
                list.clone_into(&mut residue);
                residue.remove(index);
                let size = item + calc(&residue, weight - item, mem);

                if size <= weight {
                    max = max.max(size)
                }
            }

            if item == weight {
                max = weight;
                break;
            }
        }
        mem[weight as usize] = max;
        max
    }

    let mut mem = vec![-1; weight as usize + 1];
    calc(&list, weight, &mut mem)
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

    #[test]
    fn test_bag_dfs_1() {
        let weight = 10;
        let list = vec![3, 4, 5, 7];
        let result = bag_dfs_1(list, weight);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_bag_mem() {
        let weight = 10;
        let list = vec![3, 4, 5, 7];
        let result = bag_mem(list, weight);
        assert_eq!(result, 10);
    }
}