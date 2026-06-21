/// LeetCode #2016 - Maximum Difference Between Increasing Elements
fn maximum_difference(nums: Vec<i32>) -> i32 {
    let mut mi = i32::MAX;
    let mut ans = -1;
    for x in nums {
        if x > mi {
            ans = ans.max(x - mi);
        } else {
            mi = x;
        }
    }
    ans
}

fn main() {
    println!("{}", maximum_difference(vec![7, 1, 5, 4]));
}

#[cfg(test)]
mod tests {
    use super::maximum_difference;

    #[test]
    fn example_one() {
        assert_eq!(maximum_difference(vec![7, 1, 5, 4]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_difference(vec![9, 4, 3, 2]), -1);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_difference(vec![1, 5, 2, 10]), 9);
    }
}
