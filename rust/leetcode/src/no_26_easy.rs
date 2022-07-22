/// ### \[26\] 删除有序数组中的重复项
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut temp = nums[0];
    let mut index = 0;

    for i in 1..nums.len() {
        if temp != nums[i] {
            nums[index] = temp;
            temp = nums[i];
            index += 1;
        }
    }

    if temp != nums[index] {
        nums[index] = temp;
        index += 1;
    }

    index as i32
}

#[cfg(test)]
mod tests {
    use crate::no_26_easy::remove_duplicates;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2, 2, 3, 3, 4, 5, 7, 7, 7];
        let len = remove_duplicates(&mut nums);
        println!("{:?}", &nums[0..(len as usize)]);
        assert_eq!(len, 6);
    }
}
