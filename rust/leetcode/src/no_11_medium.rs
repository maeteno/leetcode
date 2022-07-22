// 暴力破解时间不够，使用双指针
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let len = height.len();

    for x in 0..len {
        for y in x + 1..len {
            let length = (y - x) as i32;
            let size = length * if height[x] > height[y] { height[y] } else { height[x] };
            max = if size > max { size } else { max };
        }
    }

    max
}

pub fn max_area_2(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let len = height.len();
    let (mut left, mut right) = (0 as usize, len - 1 as usize);

    while left < right {
        let size = (right - left) as i32 * if height[left] > height[right] { height[right] } else { height[left] };
        max = if size > max { size } else { max };

        if height[left] > height[right] {
            right -= 1;
        } else {
            left += 1;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use crate::no_11_medium::{max_area, max_area_2};

    #[test]
    fn test_max_area() {
        let max = max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        assert_eq!(max, 49);
    }

    #[test]
    fn test_max_area_2() {
        let max = max_area_2(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        assert_eq!(max, 49);
    }
}