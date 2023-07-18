/// 10 20 不能拆分
/// 30 40 无效数据
/// 0开头的无效数据
pub fn num_decoding_ai(s: String) -> i32 {
    let mut dp = vec![0; s.len() + 1];
    dp[0] = 1;
    let chars: Vec<char> = s.chars().collect();
    for i in 1..=s.len() {
        if chars[i - 1] != '0' {
            dp[i] += dp[i - 1];
        }
        if i > 1 && chars[i - 2] != '0' && (chars[i - 2] as u8 - '0' as u8) * 10 + (chars[i - 1] as u8 - '0' as u8) <= 26 {
            dp[i] += dp[i - 2];
        }
    }
    dp[s.len()]
}


pub fn mum_decoding(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();

    // TODO： 边界检查
    fn calc(chars: &Vec<char>, n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }

        let a = chars[n as usize - 1];
        if a == '0' {
            return calc(chars, n - 2) + 1;
        }

        let two = (chars[n as usize - 1] as u8 - '0' as u8) * 10 + (chars[n as usize - 1] as u8 - '0' as u8);
        if a != '0' && two <= 26 {
            return calc(chars, n - 2) + calc(chars, n - 1) + 1;
        }

        calc(chars, n - 1) + 1
    }

    calc(&chars, s.len() as i32)
}


#[cfg(test)]
mod tests {
    use crate::num_decode::{mum_decoding, num_decoding_ai};

    #[test]
    fn test_num_decoding_ai() {
        let str = String::from("226");
        let count = num_decoding_ai(str);
        assert_eq!(count, 3)
    }

    #[test]
    fn test_num_decoding() {
        let str = String::from("226");
        let count = mum_decoding(str);
        assert_eq!(count, 3)
    }

    #[test]
    fn test_num_decoding_2() {
        let str = String::from("0226");
        let count = mum_decoding(str);
        assert_eq!(count, 3)
    }
}