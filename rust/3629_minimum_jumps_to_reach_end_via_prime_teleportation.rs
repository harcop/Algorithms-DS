/// LeetCode #3629 - Minimum Jumps to Reach End via Prime Teleportation
use std::collections::{HashMap, VecDeque};

fn build_factors() -> Vec<Vec<i32>> {
    const MX: usize = 1_000_001;
    let mut factors = vec![Vec::new(); MX];
    for i in 2..MX {
        if factors[i].is_empty() {
            let mut j = i;
            while j < MX {
                factors[j].push(i as i32);
                j += i;
            }
        }
    }
    factors
}

fn min_jumps(nums: Vec<i32>) -> i32 {
    let factors = build_factors();
    let n = nums.len();
    let mut g: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        for &p in &factors[x as usize] {
            g.entry(p).or_default().push(i);
        }
    }
    let mut ans = 0;
    let mut vis = vec![false; n];
    vis[0] = true;
    let mut q = VecDeque::from([0usize]);
    loop {
        let mut nq = VecDeque::new();
        while let Some(i) = q.pop_front() {
            if i == n - 1 {
                return ans;
            }
            let mut idx = g.remove(&nums[i]).unwrap_or_default();
            if i + 1 < n {
                idx.push(i + 1);
            }
            if i > 0 {
                idx.push(i - 1);
            }
            for j in idx {
                if j < n && !vis[j] {
                    vis[j] = true;
                    nq.push_back(j);
                }
            }
        }
        q = nq;
        ans += 1;
    }
}

fn main() {
    println!("{}", min_jumps(vec![1, 2, 4, 6]));
}

#[cfg(test)]
mod tests {
    use super::min_jumps;

    #[test]
    fn example1() {
        assert_eq!(min_jumps(vec![1, 2, 4, 6]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_jumps(vec![2, 3, 4, 7, 9]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_jumps(vec![4, 6, 5, 8]), 3);
    }
}
