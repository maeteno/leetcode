use std::collections::HashMap;

/// ####  \[3\] 无重复字符的最长子串
///
/// 给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
pub fn length_of_longest_substring(s: String) -> i32 {
    let (mut ans, mut cnt) = (0, 0);
    let mut map = HashMap::new();
    let s = s.chars().collect::<Vec<_>>();
    let mut l = 0;

    s.iter()
        .enumerate()
        .for_each(|(i, c)| {
            match map.get(c) {
                None => {
                    cnt += 1;
                    ans = ans.max(cnt);
                }
                Some(&i) => {
                    for c in &s[l..=i] {
                        map.remove(c);
                    }
                    cnt -= i - l;
                    l = i + 1;
                }
            }
            map.insert(*c, i);
        });
    ans as i32
}

#[cfg(test)]
mod test {
    use crate::no_03_medium::length_of_longest_substring;

    #[test]
    fn test_length_of_longest_substring() {
        let len = length_of_longest_substring(String::from("test_length_of_longest_substring"));
        assert_eq!(len, 9);
    }
}