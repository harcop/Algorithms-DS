/// LeetCode #3762 - Minimum Operations to Equalize Subarrays
fn min_operations(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let k = k as i64;
    queries
        .into_iter()
        .map(|q| {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let rem = (nums[l] as i64).rem_euclid(k);
            let mut vals = Vec::with_capacity(r - l + 1);
            for i in l..=r {
                let x = nums[i] as i64;
                if x.rem_euclid(k) != rem {
                    return -1;
                }
                vals.push(x / k);
            }
            vals.sort_unstable();
            let med = vals[vals.len() / 2];
            vals.iter().map(|v| (v - med).abs()).sum()
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        min_operations(vec![1, 4, 7], 3, vec![vec![0, 1], vec![0, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(
            min_operations(vec![1, 4, 7], 3, vec![vec![0, 1], vec![0, 2]]),
            vec![1, 2]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_operations(
                vec![1, 2, 4],
                2,
                vec![vec![0, 2], vec![0, 0], vec![1, 2]]
            ),
            vec![-1, 0, 1]
        );
    }
}
