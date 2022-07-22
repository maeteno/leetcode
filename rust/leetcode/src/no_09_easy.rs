/// ###  \[09\] 回文数
///
/// 给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。
/// 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
/// - 例如，121 是回文，而 123 不是。`
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut x1 = x;
    let mut x2 = 0;

    while x1 != 0 {
        x2 = (x2 * 10) + (x1 % 10);
        x1 = x1 / 10;
    }

    println!("{}:{}", x, x2);

    return x2 == x;
}

#[cfg(test)]
mod tests {
    use crate::no_09_easy::is_palindrome;

    #[test]
    fn test_add_two_numbers() {
        let result = is_palindrome(9000);
        assert_eq!(result, false);

        let result = is_palindrome(12321);
        assert_eq!(result, true);

        let result = is_palindrome(-12321);
        assert_eq!(result, false);
    }
}
