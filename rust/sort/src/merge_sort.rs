//! ## 归并排序（Merge Sort）

/// ### 归并排序
/// ![](https://images2017.cnblogs.com/blog/849589/201710/849589-20171015230557043-37375010.gif)
/// 归并排序是建立在归并操作上的一种有效的排序算法。该算法是采用分治法（Divide and Conquer）的一个非常典型的应用。将已有序的子序列合并，得到完全有序的序列；即先使每个子序列有序，再使子序列段间有序。若将两个有序表合并成一个有序表，称为2-路归并。
///
/// #### 算法描述
/// 1. 把长度为n的输入序列分成两个长度为n/2的子序列；
/// 2. 对这两个子序列分别采用归并排序；
/// 3. 将两个排序好的子序列合并成一个最终的排序序列。
/// 4. 将两个排序好的子序列合并成一个最终的排序序列。
///
/// 该算法非常适合 Java Fork/Join 框架
pub fn merge_sort(list: Vec<i32>) -> Vec<i32> {
    let len = list.len();

    if len < 2 {
        return list;
    }

    let (left, right) = slice_vec(list);

    return merge(merge_sort(left), merge_sort(right));
}

/// 分隔list
fn slice_vec(list: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let len = list.len();
    let middle = len / 2;

    let left = list[0..middle].to_vec();
    let right = list[middle..len].to_vec();

    (left, right)
}


fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut total: Vec<i32> = Vec::new();

    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left.len() && right_index < right.len() {
        let left_val = left[left_index];
        let right_val = right[right_index];

        if left_val < right_val {
            total.push(left_val);
            left_index += 1;
        } else {
            total.push(right_val);
            right_index += 1;
        }
    }

    while left_index < left.len() {
        total.push(left[left_index]);
        left_index += 1;
    }

    while right_index < right.len() {
        total.push(right[right_index]);
        right_index += 1;
    }

    total
}

/// 分隔list
fn _bisect(list: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut reght: Vec<i32> = Vec::new();

    let len = list.len();
    let middle = len / 2;

    for index in 0..len {
        let item = list[index];
        if index < middle {
            left.push(item);
        } else {
            reght.push(item)
        }
    }

    (left, reght)
}


#[cfg(test)]
mod test {
    use crate::merge_sort::merge_sort;

    #[test]
    fn test_merge_sort() {
        let list = vec![0, 2, 6, 5, 3, 9, 8, 6, 3, 0, 4, 7, 5, 1, 100, 89];
        let result = merge_sort(list);

        println!("{:?}", result);
        assert_eq!(result[0], 0);
    }
}
