/// #### \[01\] 两数之和
///
/// > 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
/// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
/// 你可以按任意顺序返回答案。
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let len = nums.len();
    let mut index = 0;

    while index < len {
        let val1 = nums[index];
        let val2 = target - val1;

        let mut index2 = index + 1;
        while index2 < len {
            let temp = nums[index2];
            if temp == val2 {
                return vec![index as i32, index2 as i32];
            }
            index2 += 1;
        }

        index += 1;
    }

    return vec![];
}

#[cfg(test)]
mod tests {
    use crate::no_01_easy::two_sum;

    #[test]
    fn test_two_sum() {
        let nums = vec![1, 2, 3, 4, 5];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![3, 4]);
    }
}
