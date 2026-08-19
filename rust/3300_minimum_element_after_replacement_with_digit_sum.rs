/// LeetCode #3300 - Minimum Element After Replacement With Digit Sum
fn min_element(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .map(|mut x| {
            let mut y = 0;
            while x > 0 {
                y += x % 10;
                x /= 10;
            }
            y
        })
        .min()
        .unwrap()
}

fn main() {
    println!("{}", min_element(vec![10, 12, 13, 14]));
}

#[cfg(test)]
mod tests {
    use super::min_element;

    #[test]
    fn example1() {
        assert_eq!(min_element(vec![10, 12, 13, 14]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_element(vec![1, 2, 3, 4]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_element(vec![999, 19, 199]), 10);
    }
}
