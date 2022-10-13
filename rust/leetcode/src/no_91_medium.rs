//！ 91. 解码方法
//! <https://leetcode.cn/problems/decode-ways/>


pub fn num_decodings(s: String) -> i32 {
    let nums: Vec<u8> = s.into_bytes().iter()
        .map(|x| { x - 48 })
        .collect();

    if nums.len() == 1 {
        return 1;
    }

    if nums.len() > 0 && nums[0] == 0 {
        return 0;
    }

    let mut count = 0;
    decode_1(nums, &mut count);
    count
}


// 回溯
// 缺少边界检查
pub fn decode_1(nums: Vec<u8>, count: &mut i32) {
    let len = nums.len();

    if len == 0 {
        return;
    }

    if nums[0] == 0 {
        return;
    }

    if len == 2 && nums[1] == 0 {
        *count = *count + 1;
        return;
    }

    if len == 1 {
        *count = *count + 1;
        return;
    }

    if len >= 2 && nums[1] == 0 {
        let x = nums[2..len].to_vec();
        decode_1(x, count);
    } else {
        // 过滤第一个数字
        let x = nums[1..len].to_vec();
        decode_1(x, count);

        if nums[0] <= 2 && nums[0] * 10 + nums[1] <= 26 {
            let x = nums[2..len].to_vec();
            decode_1(x, count);
        }
    }
}

pub fn num_decoding_2(s: String) -> i32 {
    let nums: Vec<u8> = s.into_bytes().iter()
        .map(|x| { x - 48 })
        .collect();

    let len = nums.len();

    if len == 1 {
        return 1;
    }

    if len > 0 && nums[0] == 0 {
        return 0;
    }

    0
}

pub fn num_decoding_3(s: String) -> i32 {
    let nums: Vec<u8> = s.into_bytes().iter()
        .map(|x| { x - 48 })
        .collect();

    let len = nums.len();

    if len > 0 && nums[0] == 0 {
        return 0;
    }

    if len == 1 {
        return 1;
    }

    num_decoding_3_1(&nums, 0, 0)
}

pub fn num_decoding_3_1(nums: &Vec<u8>, index: usize, mut count: i32) -> i32 {
    let len = nums.len();

    if index >= len {
        return count + 1;
    }

    if nums[index] == 0 {
        return count;
    }

    if len - index <= 1 {
        return count + 1;
    }

    let tow = (nums[index] * 10) + nums[index + 1];

    if tow <= 26 {
        if nums[index + 1] == 0 {
            count = num_decoding_3_1(nums, index + 2, count);
        } else {
            count = num_decoding_3_1(nums, index + 1, count);
            count = num_decoding_3_1(nums, index + 2, count);
        }
    } else {
        count = num_decoding_3_1(nums, index + 1, count);
    }

    return count;
}

#[cfg(test)]
mod tests {
    use crate::no_91_medium::{num_decoding_3, num_decodings};

    #[test]
    fn test_num_decodings() {
        let count = num_decodings(String::from("12"));
        println!("{}", count)
    }

    #[test]
    fn test_num_decodings_2() {
        let count = num_decodings(String::from("121323279432148326710792043162710374215372192041032142032141032483247328964782397"));
        println!("{}", count)
    }

    #[test]
    fn test_num_decodings_3() {
        let count = num_decodings(String::from("1201"));
        println!("{}", count)
    }

    #[test]
    fn test_num_num_decoding_3() {
        let count = num_decoding_3(String::from("1201"));
        println!("{}", count)
    }

    #[test]
    fn test_num_num_decoding_3_2() {
        let count = num_decoding_3(String::from("111111111111111111111111111111111111111111111"));
        println!("{}", count)
    }
}


