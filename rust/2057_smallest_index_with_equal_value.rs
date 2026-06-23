/// LeetCode #2057 - Smallest Index With Equal Value
fn smallest_equal(nums: Vec<i32>) -> i32 {
    for (i, &x) in nums.iter().enumerate() {
        if (i % 10) as i32 == x {
            return i as i32;
        }
    }
    -1
}

fn main() {
    println!("{}", smallest_equal(vec![0, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::smallest_equal;

    #[test]
    fn example_one() {
        assert_eq!(smallest_equal(vec![0, 1, 2]), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_equal(vec![4, 3, 2, 1]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(smallest_equal(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), -1);
    }
}
