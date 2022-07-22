//! ## 快速排序（Quick Sort）

/// ### 快速排序
/// 快速排序的基本思想：通过一趟排序将待排记录分隔成独立的两部分，其中一部分记录的关键字均比另一部分的关键字小，则可分别对这两部分记录继续进行排序，以达到整个序列有序。
/// #### 算法描述
/// 快速排序使用分治法来把一个串（list）分为两个子串（sub-lists）。具体算法描述如下：
/// 1. 从数列中挑出一个元素，称为 “基准”（pivot）；
/// 2. 重新排序数列，所有元素比基准值小的摆放在基准前面，所有元素比基准值大的摆在基准的后面（相同的数可以到任一边）。在这个分区退出之后，该基准就处于数列的中间位置。这个称为分区（partition）操作；
/// 3. 递归地（recursive）把小于基准值元素的子数列和大于基准值元素的子数列排序。
/// ![](https://images2017.cnblogs.com/blog/849589/201710/849589-20171015230936371-1413523412.gif)
pub fn quick_sort(list: &mut Vec<i32>, left: usize, right: usize) {
    if left < right {
        let partition_index = partition(list, left, right);
        quick_sort(list, left, partition_index - 1);
        quick_sort(list, partition_index + 1, right);
    }
}

// 1. 取第一个数，比这个数大的放右边。小的放左边
// 2. 再分别比较着两边的数据
fn partition(list: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let mut index = left + 1;

    for i in index..=right {
        if list[i] < list[left] {
            list.swap(i, index);
            index += 1;
        }
    }

    list.swap(left, index - 1);
    index - 1
}


#[cfg(test)]
mod tests {
    use crate::quick_sort::quick_sort;

    #[test]
    fn test_quick_sort() {
        let mut list = vec![1, 3, 6, 9, 5, 2, 8, 4, 6, 7, 0, 3, 1, 3];
        let right = list.len() - 1;
        quick_sort(&mut list, 0, right);

        println!("{:?}", list);
    }
}