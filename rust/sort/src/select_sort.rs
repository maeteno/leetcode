/// ### 选择排序
/// 找出值最新的索引，交互移动到最前面
/// ![](https://images2017.cnblogs.com/blog/849589/201710/849589-20171015224719590-1433219824.gif)
pub fn select_sort(list: &mut Vec<i32>) {
    for x in 0..list.len() {
        let mut min_index = x;
        for y in x..list.len() {
            if list[y] < list[min_index] {
                min_index = y;
            }
        }

        if min_index != x {
            list.swap(x, min_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::select_sort::select_sort;

    #[test]
    fn test_select_sort() {
        let mut list = vec![5, 6, 9, 8, 3, 4, 0, 7, 2, 1];
        select_sort(&mut list);
        println!("{:?}", list);

        assert_eq!(list[0], 0);
    }
}
