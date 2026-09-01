/// LeetCode #3550 - Smallest Index With Digit Sum Equal to Index
fn smallest_index(nums: Vec<i32>) -> i32 {
    for (i, mut x) in nums.into_iter().enumerate() {
        let mut s = 0;
        while x > 0 {
            s += x % 10;
            x /= 10;
        }
        if s == i as i32 {
            return i as i32;
        }
    }
    -1
}

fn main() {
    println!("{}", smallest_index(vec![1, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::smallest_index;

    #[test]
    fn example1() {
        assert_eq!(smallest_index(vec![1, 3, 2]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(smallest_index(vec![1, 10, 11]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(smallest_index(vec![1, 2, 3]), -1);
    }
}
