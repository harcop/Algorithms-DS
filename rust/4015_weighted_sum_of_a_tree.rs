/// LeetCode #4015 - Weighted Sum of a Tree
fn weighted_sum(parent: Vec<i32>, nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut g = vec![Vec::new(); n];
    for i in 1..n {
        g[parent[i] as usize].push(i);
    }
    let mut ans = 0i64;
    let mut q = vec![0usize];
    let mut d = 0i64;
    while !q.is_empty() {
        d += 1;
        let mut nq = Vec::new();
        for i in q {
            ans += nums[i] as i64 * (1 - d);
            nq.extend_from_slice(&g[i]);
        }
        q = nq;
    }
    let sum: i64 = nums.iter().map(|&x| x as i64).sum();
    ans + d * sum
}

fn main() {
    println!(
        "{}",
        weighted_sum(vec![-1, 0, 0, 0, 2, 2], vec![5, 2, 3, 1, 4, 6])
    );
}

#[cfg(test)]
mod tests {
    use super::weighted_sum;

    #[test]
    fn example1() {
        assert_eq!(
            weighted_sum(vec![-1, 0, 0, 0, 2, 2], vec![5, 2, 3, 1, 4, 6]),
            37
        );
    }

    #[test]
    fn example2() {
        assert_eq!(weighted_sum(vec![-1, 0, 1, 2], vec![1, 2, 3, 4]), 20);
    }
}
