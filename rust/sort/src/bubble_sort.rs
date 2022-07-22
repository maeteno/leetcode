//! 冒泡排序

// /// ### 冒泡排序
// /// ![](https://images2017.cnblogs.com/blog/849589/201710/849589-20171015223238449-2146169197.gif)
pub fn bubble_sort(arr: &mut Vec<i32>) {
    for x in (0..arr.len()).rev() {
        for y in 0..x {
            if arr[y] > arr[y + 1] {
                arr.swap(y, y + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bubble_sort::{bubble_sort};

    #[test]
    fn test_bubble_vec() {
        let mut list: Vec<i32> = vec![2, 3, 4, 6, 7, 9, 0, 8, 1, -1];

        bubble_sort(&mut list);
        print!("{:?}", list);
    }
}
