/// LeetCode #3695 - Maximize Alternating Sum Using Swaps
use std::collections::HashMap;

fn max_alternating_sum(nums: Vec<i32>, swaps: Vec<Vec<i32>>) -> i64 {
    let n = nums.len();
    let mut parent: Vec<usize> = (0..n).collect();

    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    for s in &swaps {
        let a = find(&mut parent, s[0] as usize);
        let b = find(&mut parent, s[1] as usize);
        if a != b {
            parent[a] = b;
        }
    }

    let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let r = find(&mut parent, i);
        groups.entry(r).or_default().push(i);
    }

    let mut ans = 0i64;
    for indices in groups.values() {
        let mut vals: Vec<i32> = indices.iter().map(|&i| nums[i]).collect();
        let even_count = indices.iter().filter(|&&i| i % 2 == 0).count();
        vals.sort_unstable_by(|a, b| b.cmp(a));
        let sum_all: i64 = vals.iter().map(|&x| x as i64).sum();
        let sum_top: i64 = vals.iter().take(even_count).map(|&x| x as i64).sum();
        ans += 2 * sum_top - sum_all;
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_alternating_sum(vec![1, 2, 3], vec![vec![0, 2], vec![1, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::max_alternating_sum;

    #[test]
    fn example1() {
        assert_eq!(
            max_alternating_sum(vec![1, 2, 3], vec![vec![0, 2], vec![1, 2]]),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(max_alternating_sum(vec![1, 2, 3], vec![vec![1, 2]]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(
            max_alternating_sum(vec![1, 1000000000, 1, 1000000000, 1, 1000000000], vec![]),
            -2999999997
        );
    }
}
