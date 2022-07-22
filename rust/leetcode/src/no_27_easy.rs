/// ### [\27\] 移除元素
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let len = nums.len();

    if len <= 0 {
        return 0;
    }

    let mut index = 1;
    let mut new_index = 0;
    let temp = nums[0];

    while index < len {
        let it = nums[index];

        if it != val {
            nums[new_index] = nums[index];
            new_index += 1;
        }

        index += 1;
    }

    if temp != val {
        nums[new_index] = temp;
        new_index += 1;
    }

    return new_index as i32;
}

pub fn remove_element_2(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut index = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[index] = nums[i];
            index += 1;
        }
    }
    return index as i32;
}

#[cfg(test)]
mod tests {
    use crate::no_27_easy::remove_element;
    use crate::no_27_easy::remove_element_2;

    #[test]
    fn test_remove_element_2() {
        let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        let target = 5;
        let result = remove_element_2(&mut nums, target);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_remove_element() {
        let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5, 5, 5, 6];
        let target = 5;
        let result = remove_element(&mut nums, target);
        println!("{:?}", nums);
        assert_eq!(result, 5);
    }
}
