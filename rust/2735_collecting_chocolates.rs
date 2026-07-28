/// LeetCode #2735 - Collecting Chocolates
fn min_cost(nums: Vec<i32>, x: i32) -> i64 {
    let n = nums.len();
    let mut f = vec![vec![0; n]; n];
    for i in 0..n {
        f[i][0] = nums[i];
        for j in 1..n {
            f[i][j] = f[i][j - 1].min(nums[(i + n - j) % n]);
        }
    }
    let mut ans = i64::MAX;
    for j in 0..n {
        let mut cost = (x as i64) * (j as i64);
        for i in 0..n {
            cost += f[i][j] as i64;
        }
        ans = ans.min(cost);
    }
    ans
}

fn main() {
    println!("{}", min_cost(vec![20, 1, 15], 5));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example_one() {
        assert_eq!(min_cost(vec![20, 1, 15], 5), 13);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_cost(vec![1, 2, 3], 4), 6);
    }
}
