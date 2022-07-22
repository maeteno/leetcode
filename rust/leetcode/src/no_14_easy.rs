/// 14. 最长公共前缀
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    strs.iter()
        .max()
        .unwrap()
        .chars()
        .zip(strs.iter().min().unwrap().chars())
        .take_while(|x| x.0 == x.1)
        .map(|x| x.0)
        .collect()
}

pub fn longest_common_prefix_2(strs: Vec<String>) -> String {
    // 获取一个最大值和最小值，这里的max和min。和长度无关，是根据字典序进行排序的。只需要比较这两个即可获取最大的公共前缀
    let max = strs.iter().max();
    let min = strs.iter().min();
    println!("max: {:?}", max.unwrap());
    println!("min: {:?}", min.unwrap());

    // unwrap 是对Option类型的解包。chars 是返回对字符串字符的迭代器
    let max_chars = max.unwrap().chars();
    println!("max chars: {:?}", max_chars);
    let min_chars = min.unwrap().chars();
    println!("min chars: {:?}", min_chars);

    // zip 将两个集合类型的数据合并，返回一个元组。超出的部分会忽略
    let zip = max_chars.zip(min_chars);
    println!("zip: {:?}", zip);

    // filter 和 take_while 两个函数的不同点在于.filter 会返回所有结果为true的item，只是跳过值为false的结果。
    // take_while 是当返回false时，会停止获取后续的数据。不会继续后面的操作，filter不会在遇到false停止，只是跳过
    // 可以理解为 take_while 是具有短路操作的 filter
    let take = zip.take_while(|x| x.0 == x.1);
    let map = take.map(|x| x.0);

    map.collect()
}

#[cfg(test)]
mod tests {
    use crate::no_14_easy::{longest_common_prefix, longest_common_prefix_2};

    #[test]
    fn test_longest_common_prefix() {
        let strs = vec![
            String::from("abdcdefggg"),
            String::from("abbdefeee"),
            String::from("abc"),
        ];
        let result = longest_common_prefix(strs);
        println!("{}", result);
        assert_eq!(result, String::from("ab"));
    }

    #[test]
    fn test_longest_common_prefix_2() {
        println!("{}", 123);
        let strs = vec![
            String::from("abcd"),
            String::from("abbdefeee"),
            String::from("abzdefe"),
        ];
        let result = longest_common_prefix_2(strs);
        println!("{}", result);
        assert_eq!(result, String::from("ab"));
    }
}
