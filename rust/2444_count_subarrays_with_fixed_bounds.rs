/// LeetCode #2444 - Count Subarrays With Fixed Bounds
fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let mut last_invalid = -1i64;
    let mut last_min = -1i64;
    let mut last_max = -1i64;
    let mut answer = 0i64;

    for (index, num) in nums.into_iter().enumerate() {
        let index = index as i64;
        if num < min_k || num > max_k {
            last_invalid = index;
        }
        if num == min_k {
            last_min = index;
        }
        if num == max_k {
            last_max = index;
        }
        answer += (last_min.min(last_max) - last_invalid).max(0);
    }

    answer
}

fn main() {
    println!("{}", count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5));
}

#[cfg(test)]
mod tests {
    use super::count_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5), 2);
    }

    #[test]
    fn equal_bounds() {
        assert_eq!(count_subarrays(vec![1, 1, 1, 1], 1, 1), 10);
    }
}
