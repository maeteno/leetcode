//! ## 插入排查

pub fn insertion_sort(list: &mut Vec<i32>) {
    for x in 1..list.len() {
        let current = list[x];
        for y in (0..x).rev() {
            if list[y] < current {
                // 将需要插入的数据插入到当前位置的后面
                list[y + 1] = current;
                break;
            } else {
                // 将数据往后移动
                list[y + 1] = list[y];

                if y == 0 {
                    list[0] = current;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::insertion_sort::insertion_sort;

    #[test]
    fn test_insertion_sort() {
        let mut list = vec![1, 5, 6, 4, 7, 3, 2, 8, 9, 0, 10, -1];
        insertion_sort(&mut list);
        println!("{:?}", list);

        assert_eq!(list[0], -1);
    }
}
