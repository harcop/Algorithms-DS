/// LeetCode #3018 - Maximum Number of Removal Queries That Can Be Processed I
fn maximum_removal_queries(nums: Vec<i32>, queries: Vec<i32>) -> i32 {
    let n = nums.len();
    let m = queries.len();
    let mut f = vec![vec![0usize; n]; n];

    for i in 0..n {
        for j in (i..n).rev() {
            if i > 0 {
                let idx = f[i - 1][j];
                if idx < m && nums[i - 1] >= queries[idx] {
                    f[i][j] = f[i][j].max(idx + 1);
                }
            }
            if j + 1 < n {
                let idx = f[i][j + 1];
                if idx < m && nums[j + 1] >= queries[idx] {
                    f[i][j] = f[i][j].max(idx + 1);
                }
            }
            if f[i][j] == m {
                return m as i32;
            }
        }
    }

    (0..n)
        .map(|i| {
            let idx = f[i][i];
            if idx < m && nums[i] >= queries[idx] {
                idx + 1
            } else {
                idx
            }
        })
        .max()
        .unwrap_or(0) as i32
}

fn main() {
    println!(
        "{}",
        maximum_removal_queries(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 6])
    );
    println!("{}", maximum_removal_queries(vec![2, 3, 2], vec![2, 2, 3]));
    println!("{}", maximum_removal_queries(vec![3, 4, 3], vec![4, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::maximum_removal_queries;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_removal_queries(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 6]),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_removal_queries(vec![2, 3, 2], vec![2, 2, 3]), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_removal_queries(vec![3, 4, 3], vec![4, 3, 2]), 2);
    }
}
