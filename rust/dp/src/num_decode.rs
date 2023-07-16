/// 10 20 不能拆分
/// 30 40 无效数据
/// 0开头的无效数据
pub fn num_decodings(s: String) -> i32 {
    if s[0..1].parse::<i32>().unwrap() == 0 {
        return 0;
    }

    fn calc(s: &str) -> i32 {
        return if s.len() == 0 {
            0
        } else if s.len() == 1 {
            1
        } else {
            let after = &s[s.len() - 2..s.len()];
            let num = after.parse::<i32>().unwrap();


            if num == 10 {
                return calc(&s[0..s.len() - 2]) + 1;
            }

            if num <= 26 {
                return calc(&s[0..s.len() - 2]) + calc(&s[0..s.len() - 1]) + 1;
            }

            calc(&s[0..s.len() - 1]) + 1
        };
    }

    calc(&s[0..s.len()])
}

#[cfg(test)]
mod tests {
    use crate::num_decode::num_decodings;

    #[test]
    fn test_num_decodings() {
        let str = String::from("226");
        let count = num_decodings(str);
        assert_eq!(count, 3)
    }

    #[test]
    fn test_num_decodings_1() {
        let str = String::from("12");
        let count = num_decodings(str);
        assert_eq!(count, 2)
    }
}