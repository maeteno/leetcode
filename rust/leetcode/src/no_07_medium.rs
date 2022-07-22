pub fn reverse(x: i32) -> i32 {
    // TODO: 上下边界检查
    if x == 0 {
        return 0;
    }

    let mut raw = x;
    let tag = if x > 0 {
        1
    } else {
        raw *= -1;
        -1
    };

    let mut result = 0;
    while raw > 0 {
        result = (result * 10) + (raw % 10);
        raw /= 10;
    }

    result * tag
}


#[cfg(test)]
mod tests {
    use crate::no_07_medium::reverse;

    #[test]
    fn test_reverse() {
        let result = reverse(321);
        assert_eq!(result, 123);

        let result = reverse(-321);
        assert_eq!(result, -123);
    }
}