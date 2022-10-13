//! 给你一个非负整数 num ，请你返回将它变成 0 所需要的步数。 如果当前数字是偶数，你需要把它除以 2 ；否则，减去 1 。


// 递归
pub fn number_of_steps(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }

    if num % 2 == 0 {
        number_of_steps(num / 2) + 1
    } else {
        number_of_steps(num - 1) + 1
    }
}

// 递推
pub fn number_of_steps_2(num: i32) -> i32 {
    let mut count = 0;
    let mut num = num;

    while num != 0 {
        num = if num % 2 == 0 {
            num / 2
        } else {
            num - 1
        };

        count += 1;
    }

    return count;
}

// 递推
pub fn number_of_steps_3(num: i32) -> i32 {
    let mut mem = vec![0; num as usize + 1];

    for index in 1..=num as usize {
        if index % 2 == 0 {
            mem[index] = mem[index / 2] + 1;
        } else {
            mem[index] = mem[index - 1] + 1;
        }
    }

    mem[num as usize]
}

#[cfg(test)]
mod tests {
    use crate::no_1342_easy::{number_of_steps, number_of_steps_2, number_of_steps_3};

    #[test]
    fn test_number_of_steps() {
        let step = number_of_steps(809);

        println!("Step: {}", step)
    }

    #[test]
    fn test_number_of_steps_2() {
        let step = number_of_steps_2(809);

        println!("Step: {}", step)
    }

    #[test]
    fn test_number_of_steps_3() {
        let step = number_of_steps_3(809);

        println!("Step: {}", step)
    }
}