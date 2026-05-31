/// LeetCode #1630 - Arithmetic Subarrays
fn check_arithmetic_subarrays(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    queries
        .into_iter()
        .map(|q| {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let d = q[2];
            let mut sub: Vec<i32> = nums[l..=r].to_vec();
            sub.sort_unstable();
            if sub.len() <= 1 {
                return true;
            }
            let diff = sub[1] - sub[0];
            if diff != d {
                return false;
            }
            sub.windows(2).all(|w| w[1] - w[0] == diff)
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        check_arithmetic_subarrays(
            vec![4, 6, 5, 9, 3, 7],
            vec![vec![0, 0, 2], vec![0, 1, 2], vec![1, 3, 2]],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::check_arithmetic_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(
            check_arithmetic_subarrays(
                vec![4, 6, 5, 9, 3, 7],
                vec![vec![0, 0, 2], vec![0, 1, 2], vec![1, 3, 2]],
            ),
            vec![true, true, false]
        );
    }
}
