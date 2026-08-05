/// LeetCode #3010 - Divide an Array Into Subarrays With Minimum Cost I
fn minimum_cost(nums: Vec<i32>) -> i32 {
    let mut rest: Vec<i32> = nums[1..].to_vec();
    rest.sort_unstable();
    nums[0] + rest[0] + rest[1]
}

fn main() {
    println!("{}", minimum_cost(vec![1, 2, 3, 12]));
    println!("{}", minimum_cost(vec![5, 4, 3]));
    println!("{}", minimum_cost(vec![10, 3, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example_one() {
        assert_eq!(minimum_cost(vec![1, 2, 3, 12]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_cost(vec![5, 4, 3]), 12);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_cost(vec![10, 3, 1, 1]), 12);
    }
}
