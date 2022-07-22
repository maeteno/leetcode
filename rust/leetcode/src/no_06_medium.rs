/// 6. Z 字形变换
/// 将一个给定字符串 s 根据给定的行数 numRows ，以从上往下、从左到右进行 Z 字形排列。
///
/// 比如输入字符串为 "PAYPALISHIRING" 行数为 3 时，排列如下：
pub fn convert(s: String, num_rows: i32) -> String {
    let len = s.len();

    if len <= 1 || num_rows <= 1 || len <= num_rows as usize {
        return s;
    }

    let char_list: Vec<char> = s.chars().collect();

    // 2N-2-offset
    let mut offset: usize = 0;

    let mut result: Vec<char> = Vec::new();
    for num_row in 0..num_rows {
        let mut s_index = num_row as usize;
        result.push(char_list[s_index]);

        let span = (num_rows as usize) * 2 - 2 - offset;
        let span2 = offset;
        while s_index < len {
            if span != 0 {
                s_index += span;

                if s_index >= len {
                    continue;
                }
                result.push(char_list[s_index]);
            }

            if span2 != 0 {
                s_index += span2;
                if s_index >= len {
                    continue;
                }
                result.push(char_list[s_index]);
            }
        }

        offset += 2;
    }

    result.iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::no_06_medium::convert;

    #[test]
    fn test_convert() {
        let result = convert(String::from("PAYPALISHIRING"), 3);
        println!("{}", result);

        assert_eq!(result, "PAHNAPLSIIGYIR");
    }
}