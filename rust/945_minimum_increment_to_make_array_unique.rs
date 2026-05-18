/// LeetCode #945 - Minimum Increment to Make Array Unique

fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut moves = 0i64;
    let mut need = nums[0];
    for &x in &nums {
        if x < need {
            moves += (need - x) as i64;
        } else {
            need = x;
        }
        need += 1;
    }
    moves as i32
}

fn main() {
    println!("{}", min_increment_for_unique(vec![1, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::min_increment_for_unique;

    #[test]
    fn example_one() {
        assert_eq!(min_increment_for_unique(vec![1, 2, 2]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_increment_for_unique(vec![3, 2, 1, 2, 1, 7]), 6);
    }
}
