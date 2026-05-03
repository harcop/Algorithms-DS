/// LeetCode #198 - House Robber
fn rob(nums: Vec<i32>) -> i32 {
    let (mut skip, mut take) = (0i32, 0i32);
    for x in nums {
        let new_skip = skip.max(take);
        take = skip + x;
        skip = new_skip;
    }
    skip.max(take)
}

fn main() {
    println!("{}", rob(vec![2, 7, 9, 3, 1]));
}

#[cfg(test)]
mod tests {
    use super::rob;

    #[test]
    fn example_one() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
