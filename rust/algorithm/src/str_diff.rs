//！ 字符串差异比较


pub struct StrDiff {
    pub x1: Vec<u8>,
    pub x2: Vec<u8>,
    pub l1: usize,
    pub l2: usize,
    pub min_dist: usize,

}

impl StrDiff {
    pub fn new(s1: String, s2: String) -> StrDiff {
        let x1 = s1.into_bytes();
        let x2 = s2.into_bytes();
        let l1 = x1.len();
        let l2 = x2.len();

        StrDiff {
            x1,
            x2,
            l1,
            l2,
            min_dist: 65535,
        }
    }

    pub fn diff_distance(&mut self, i: usize, j: usize, mut dist: usize) {
        let StrDiff { x1, x2, l1, l2, min_dist } = self;

        if i == *l1 || j == *l2 {
            if i < *l1 { dist += *l1 - i; };
            if j < *l2 { dist += *l2 - j; };
            if dist < *min_dist { self.min_dist = dist; };
            return;
        }

        if x1[i] == x2[j] {
            self.diff_distance(i + 1, j + 1, dist);
        } else {
            self.diff_distance(i + 1, j, dist + 1); // 删除a[i]或者b[j]前添加一个字符
            self.diff_distance(i, j + 1, dist + 1); // 删除b[j]或者a[i]前添加一个字符
            self.diff_distance(i + 1, j + 1, dist + 1); // 将a[i]和b[j]替换为相同字符
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::str_diff::StrDiff;

    #[test]
    fn test_diff() {
        let mut diff = StrDiff::new(String::from("12345412321"), String::from("123532"));
        diff.diff_distance(0, 0, 0);
        println!("{:?}", diff.min_dist);
    }
}