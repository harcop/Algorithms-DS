/// LeetCode #1906 - Minimum Absolute Difference Queries
const INF: i32 = i32::MAX / 2;

fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let m = nums.len();
    let mut pre = vec![vec![0i32; 101]; m + 1];
    for i in 1..=m {
        for j in 1usize..=100 {
            let t = if nums[i - 1] == j as i32 { 1 } else { 0 };
            pre[i][j] = pre[i - 1][j] + t;
        }
    }

    let mut ans = Vec::with_capacity(queries.len());
    for q in queries {
        let left = q[0] as usize;
        let right = q[1] as usize + 1;
        let mut best = INF;
        let mut last: Option<usize> = None;
        for j in 1usize..=100 {
            if pre[right][j] - pre[left][j] > 0 {
                if let Some(prev) = last {
                    best = best.min((j - prev) as i32);
                }
                last = Some(j);
            }
        }
        ans.push(if best == INF { -1 } else { best });
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        min_difference(vec![1, 3, 4, 8], vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_difference;

    #[test]
    fn example_one() {
        assert_eq!(
            min_difference(
                vec![1, 3, 4, 8],
                vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 3]]
            ),
            vec![2, 1, 4, 1]
        );
    }
}
