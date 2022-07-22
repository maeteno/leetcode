/// Given a string s, return the longest palindromic substring in s.
///
pub fn longest_palindrome(s: String) -> String {
    let len = s.len();

    if len < 2 { s } else {
        let chars: Vec<char> = s.chars().collect();
        let (mut max_len, mut begin) = (1, 0);
        let mut dp = vec![vec![false; len]; len];

        for index in 0..len {
            dp[index][index] = true;
        }

        for index in 2..len {
            for left in 0..len {
                let right = index + left - 1;
                if right >= len {
                    break;
                }

                if chars[left] != chars[right] {
                    dp[left][right] = false;
                } else {
                    if right - left < 3 {
                        dp[left][right] = true;
                    } else {
                        dp[left][right] = dp[left + 1][right - 1];
                    }
                }

                if dp[left][right] && right - index + 1 > max_len {
                    max_len = right - left + 1;
                    begin = left;
                }
            }
        }

        print_vec(&dp);
        s[begin - 1..=begin + max_len].to_string()
    }
}

fn print_vec(dp: &Vec<Vec<bool>>) {
    let (mut left, mut right) = (0, 0);
    dp.iter().for_each(|it| {
        right = 0;
        it.iter().for_each(|value| {
            // print!("[{}][{}] => {}\t", left, right, value);
            print!("{}\t", value);
            right += 1;
        });
        left += 1;
        println!()
    })
}


#[cfg(test)]
mod tests {
    use crate::no_05_medium::longest_palindrome;

    #[test]
    fn test_longest_palindrome() {
        let sub = longest_palindrome(String::from("12c121c69"));

        assert_eq!(sub, String::from("c121c"));
    }
}
