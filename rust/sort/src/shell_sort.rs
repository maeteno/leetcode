//! ## 希尔排序

/// ### 希尔排序
/// 1959年Shell发明，第一个突破O(n2)的排序算法，是简单插入排序的改进版。它与插入排序的不同之处在于，它会优先比较距离较远的元素。希尔排序又叫缩小增量排序。
/// ![](https://images2018.cnblogs.com/blog/849589/201803/849589-20180331170017421-364506073.gif)
pub fn shell_sort(list: &mut Vec<i32>) {
    println!("{:?}", list);

    list[0] = 0;
}

#[cfg(test)]
mod tests {
    use crate::shell_sort::shell_sort;

    #[test]
    fn test_shell_sort() {
        let mut list = vec![1, 3, 6, 7, 9, 8, 0, 4, 2];
        shell_sort(&mut list);
        println!("{:?}", list);

        assert_eq!(list[0], 0);
    }
}