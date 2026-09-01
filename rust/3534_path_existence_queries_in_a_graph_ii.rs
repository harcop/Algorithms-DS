/// LeetCode #3534 - Path Existence Queries in a Graph II
fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut pairs: Vec<(i32, usize)> = nums.iter().copied().enumerate().map(|(i, x)| (x, i)).collect();
    pairs.sort_unstable();
    let m = 20usize;
    let mut f = vec![vec![0usize; m]; n];
    let mut r = n - 1;
    for l in (0..n).rev() {
        while pairs[r].0 - pairs[l].0 > max_diff {
            r -= 1;
        }
        let i = pairs[l].1;
        let j = pairs[r].1;
        f[i][0] = j;
        for k in 1..m {
            f[i][k] = f[f[i][k - 1]][k - 1];
        }
    }
    queries
        .into_iter()
        .map(|q| {
            let mut i = q[0] as usize;
            let mut j = q[1] as usize;
            if nums[i] > nums[j] {
                std::mem::swap(&mut i, &mut j);
            }
            if i == j {
                return 0;
            }
            if nums[i] == nums[j] {
                return 1;
            }
            let mut d = 0i32;
            for k in (0..m).rev() {
                if nums[f[i][k]] < nums[j] {
                    d |= 1 << k;
                    i = f[i][k];
                }
            }
            if nums[f[i][0]] < nums[j] {
                -1
            } else {
                d + 1
            }
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        path_existence_queries(5, vec![1, 8, 3, 4, 2], 3, vec![vec![0, 3], vec![2, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::path_existence_queries;

    #[test]
    fn example1() {
        assert_eq!(
            path_existence_queries(5, vec![1, 8, 3, 4, 2], 3, vec![vec![0, 3], vec![2, 4]]),
            vec![1, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            path_existence_queries(
                5,
                vec![5, 3, 1, 9, 10],
                2,
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![4, 3]]
            ),
            vec![1, 2, -1, 1]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            path_existence_queries(3, vec![3, 6, 1], 1, vec![vec![0, 0], vec![0, 1], vec![1, 2]]),
            vec![0, -1, -1]
        );
    }
}
