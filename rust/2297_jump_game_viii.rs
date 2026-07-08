/// LeetCode #2297 - Jump Game VIII
fn min_cost(nums: Vec<i32>, costs: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];

    let mut stk: Vec<usize> = Vec::new();
    for i in (0..n).rev() {
        while stk.last().map(|&j| nums[j] < nums[i]).unwrap_or(false) {
            stk.pop();
        }
        if let Some(&j) = stk.last() {
            g[i].push(j);
        }
        stk.push(i);
    }

    stk.clear();
    for i in (0..n).rev() {
        while stk.last().map(|&j| nums[j] >= nums[i]).unwrap_or(false) {
            stk.pop();
        }
        if let Some(&j) = stk.last() {
            g[i].push(j);
        }
        stk.push(i);
    }

    let mut f = vec![i64::MAX / 4; n];
    f[0] = 0;
    for i in 0..n {
        for &j in &g[i] {
            f[j] = f[j].min(f[i] + costs[j] as i64);
        }
    }
    f[n - 1]
}

fn main() {
    println!("{}", min_cost(vec![3, 2, 4, 4, 1], vec![3, 7, 6, 4, 2]));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example_one() {
        assert_eq!(min_cost(vec![3, 2, 4, 4, 1], vec![3, 7, 6, 4, 2]), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_cost(vec![0, 1, 2], vec![1, 1, 1]), 2);
    }
}
