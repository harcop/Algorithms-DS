/// LeetCode #2172 - Maximum AND Sum of Array
fn maximum_and_sum(nums: Vec<i32>, num_slots: i32) -> i32 {
    let slots = num_slots as usize;
    let mut states = 1usize;
    for _ in 0..slots {
        states *= 3;
    }

    let mut dp = vec![-1; states];
    dp[0] = 0;
    for &num in &nums {
        let mut next = vec![-1; states];
        for mask in 0..states {
            if dp[mask] < 0 {
                continue;
            }

            let mut base = 1usize;
            for slot in 1..=slots {
                let used = (mask / base) % 3;
                if used < 2 {
                    let new_mask = mask + base;
                    next[new_mask] = next[new_mask].max(dp[mask] + (num & slot as i32));
                }
                base *= 3;
            }
        }
        dp = next;
    }

    dp.into_iter().max().unwrap()
}

fn main() {
    println!("{}", maximum_and_sum(vec![1, 2, 3, 4, 5, 6], 3));
}

#[cfg(test)]
mod tests {
    use super::maximum_and_sum;

    #[test]
    fn example_one() {
        assert_eq!(maximum_and_sum(vec![1, 2, 3, 4, 5, 6], 3), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_and_sum(vec![1, 3, 10, 4, 7, 1], 9), 24);
    }
}
