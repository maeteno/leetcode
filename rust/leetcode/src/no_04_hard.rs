/// ## 4. Median of Two Sorted Arrays
/// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
///
/// The overall run time complexity should be O(log (m+n)).
/// ### 思路
/// 1. 两个数组是有序的
/// 2. 计算两个数组的长度即可计算中位数的位置
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let len1 = nums1.len();
    let len2 = nums2.len();
    let size = len1 + len2;

    // 计算中位数的位置
    let middle = size / 2;
    let (left_index, right_index) = if size % 2 == 0 {
        (middle - 1, middle)
    } else {
        (middle, middle)
    };

    let mut index_1: usize = 0;
    let mut index_2: usize = 0;

    let mut sum = 0;

    let mut left = 0;
    let mut right = 0;

    // 通过合并两个数组的同时计算出中位数的位置
    loop {
        let current = if index_1 < len1 && index_2 < len2 {
            if nums1[index_1] < nums2[index_2] {
                index_1 += 1;
                nums1[index_1 - 1]
            } else {
                index_2 += 1;
                nums2[index_2 - 1]
            }
        } else if index_1 < len1 {
            index_1 += 1;
            nums1[index_1 - 1]
        } else {
            index_2 += 1;
            nums2[index_2 - 1]
        };

        if sum == left_index {
            left = current;
        }

        if sum == right_index {
            right = current;
        }

        if sum >= right_index {
            break;
        }

        sum += 1;
    }

    (left + right) as f64 / 2.0
}

#[cfg(test)]
mod tests {
    use crate::no_04_hard::find_median_sorted_arrays;

    #[test]
    fn test_find_median_sorted_arrays() {
        let result = find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        println!("{}", result);

        assert_eq!(result, 2.5);
    }
}