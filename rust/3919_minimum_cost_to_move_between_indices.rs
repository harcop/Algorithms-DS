/// LeetCode #3919 - Minimum Cost to Move Between Indices
fn min_cost(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let n = nums.len();
    let nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
    let mut s1 = vec![0i64; n];
    let mut s2 = vec![0i64; n];
    for i in 1..n {
        let gap = nums[i] - nums[i - 1];
        let c1 = if i > 1 && nums[i - 1] - nums[i - 2] <= gap {
            gap
        } else {
            1
        };
        let c2 = if i + 1 < n && gap > nums[i + 1] - nums[i] {
            gap
        } else {
            1
        };
        s1[i] = s1[i - 1] + c1;
        s2[i] = s2[i - 1] + c2;
    }
    queries
        .into_iter()
        .map(|q| {
            let l = q[0] as usize;
            let r = q[1] as usize;
            if l < r {
                s1[r] - s1[l]
            } else {
                s2[l] - s2[r]
            }
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        min_cost(vec![-5, -2, 3], vec![vec![0, 2], vec![2, 0], vec![1, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(
            min_cost(vec![-5, -2, 3], vec![vec![0, 2], vec![2, 0], vec![1, 2]]),
            vec![6, 2, 5]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_cost(vec![0, 2, 3, 9], vec![vec![3, 0], vec![1, 2], vec![2, 0]]),
            vec![4, 1, 3]
        );
    }
}
