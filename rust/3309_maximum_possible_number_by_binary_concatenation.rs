/// LeetCode #3309 - Maximum Possible Number by Binary Concatenation
fn max_good_number(nums: Vec<i32>) -> i32 {
    fn concat(a: i32, b: i32, c: i32) -> i32 {
        let s = format!("{:b}{:b}{:b}", a, b, c);
        i32::from_str_radix(&s, 2).unwrap()
    }
    let (x, y, z) = (nums[0], nums[1], nums[2]);
    concat(x, y, z)
        .max(concat(x, z, y))
        .max(concat(y, x, z))
        .max(concat(y, z, x))
        .max(concat(z, x, y))
        .max(concat(z, y, x))
}

fn main() {
    println!("{}", max_good_number(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::max_good_number;

    #[test]
    fn example1() {
        assert_eq!(max_good_number(vec![1, 2, 3]), 30);
    }

    #[test]
    fn example2() {
        assert_eq!(max_good_number(vec![2, 8, 16]), 1296);
    }
}
