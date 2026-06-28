/// LeetCode #2134 - Minimum Swaps to Group All 1's Together II
fn min_swaps(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let ones = nums.iter().filter(|&&x| x == 1).count();
    if ones <= 1 {
        return 0;
    }

    let mut current = 0;
    let mut best = 0;
    for i in 0..2 * n {
        current += nums[i % n];
        if i >= ones {
            current -= nums[(i - ones) % n];
        }
        if i + 1 >= ones {
            best = best.max(current);
        }
    }

    ones as i32 - best
}

fn main() {
    println!("{}", min_swaps(vec![0, 1, 0, 1, 1, 0, 0]));
}

#[cfg(test)]
mod tests {
    use super::min_swaps;

    #[test]
    fn example_one() {
        assert_eq!(min_swaps(vec![0, 1, 0, 1, 1, 0, 0]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_swaps(vec![0, 1, 1, 1, 0, 0, 1, 1, 0]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_swaps(vec![1, 1, 0, 0, 1]), 0);
    }
}
