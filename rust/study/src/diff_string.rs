#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DiffInfo {
    pub val: String,
    pub status: usize,
}

/// TODO: 未完成字符串Diff
pub fn diff(new_str: String, old_str: String) -> Vec<DiffInfo> {
    vec![
        DiffInfo {
            val: new_str,
            status: 1,
        },
        DiffInfo {
            val: old_str,
            status: 1,
        },
    ]
}

#[cfg(test)]
mod tests {
    use crate::diff_string::diff;

    #[test]
    fn test_diff() {
        let new_str = String::from("my name is fu");
        let old_str = String::from("my age is 18");

        let info = diff(new_str, old_str);

        println!("{:?}", info);

        assert_eq!(1 + 1, 2);
    }
}
