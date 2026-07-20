/// LeetCode #2535 - Difference Between Element Sum and Digit Sum of an Array
fn difference_of_sum(nums: Vec<i32>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    for mut v in nums {
        x += v;
        while v > 0 {
            y += v % 10;
            v /= 10;
        }
    }
    x - y
}

fn main() {
    println!("{}", difference_of_sum(vec![1, 15, 6, 3]));
}

#[cfg(test)]
mod tests {
    use super::difference_of_sum;

    #[test]
    fn example_one() {
        assert_eq!(difference_of_sum(vec![1, 15, 6, 3]), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(difference_of_sum(vec![1, 2, 3, 4]), 0);
    }
}
