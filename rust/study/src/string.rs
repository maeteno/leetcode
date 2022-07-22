//! ## 字符串相关操作
//!
//! 支持基本的字符串操作，和其他语言不同的是需要注意所有权的问题。
//!
//! 一般性的细节处理，需要转换为字节或者字符迭代器进行操作。
//!
//! #### 基础方法
//! | 方法       | 简介                                  |
//! | ---------- | ----------------------------------- |
//! | replace()  | 搜索指定模式并替换                     |
//! | as_str()   | 将字符串对象转换为字符串字面量           |
//! | push()     | 字符串末尾追加字符                     |
//! | push_str() | 字符串末尾追加字符串                   |
//! | split()    | 根据指定模式分割字符串并返回分割后的迭代器 |
//! | chars()    | 返回字符串所有字符组成的迭代器           |
//! | bytes()    | 返回字符串所有字节组成的迭代器           |

/// ### 字符串字节遍历
pub fn str_foreach_bytes(s1: String) {
    let b1 = s1.bytes();
    // 遍历值
    b1.for_each(|val| {
        println!("{}", val);
    });

    let b2 = s1.bytes();
    // 遍历 key-value
    b2.enumerate().for_each(|(index, val)| {
        println!("{}:{}", index, val);
    });
}

/// ### 字符串字符遍历
pub fn str_foreach_chars(s1: String) {
    let c1 = s1.chars();
    // 遍历值
    c1.for_each(|val| {
        println!("{}", val);
    });

    // 遍历 key-value
    let c2 = s1.chars();
    c2.enumerate().for_each(|(index, val)| {
        println!("{}:{}", index, val);
    });

    let c3 = s1.chars();
    for val in c3 {
        println!("{}", val);
    }

    let c4 = s1.chars();
    for (index, val) in c4.enumerate() {
        println!("{}:{}", index, val);
    }
}

/// ### 字符串替换
pub fn str_replace(s1: String) {
    let s2 = s1.replace("a", "bbb");
    println!("{}", s2);
}

#[cfg(test)]
mod tests {
    use crate::string::{str_foreach_bytes, str_foreach_chars, str_replace};

    #[test]
    fn test_foreach() {
        str_foreach_bytes(String::from("abc"));
        str_foreach_chars(String::from("abc"));
    }

    #[test]
    fn test_replace() {
        str_replace(String::from("abc"));
    }
}
