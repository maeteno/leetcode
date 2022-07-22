pub fn my_atoi(s: String) -> i32 {



    println!("{}", s);
    123
}

#[cfg(test)]
mod tests {
    use crate::no_08_medium::my_atoi;

    #[test]
    fn test_my_atoi() {
        let mun = my_atoi("123".to_string());
        assert_eq!(mun, 123)
    }
}