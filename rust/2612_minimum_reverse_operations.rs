/// LeetCode #2612 - Minimum Reverse Operations
use std::collections::{BTreeSet, VecDeque};

fn min_reverse_operations(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
    let mut ans = vec![-1; n as usize];
    let mut ts = [BTreeSet::new(), BTreeSet::new()];

    for i in 0..n {
        ts[(i % 2) as usize].insert(i);
    }
    ans[p as usize] = 0;
    ts[(p % 2) as usize].remove(&p);

    for &b in &banned {
        ts[(b % 2) as usize].remove(&b);
    }

    ts[0].insert(n);
    ts[1].insert(n);
    let mut q = VecDeque::new();
    q.push_back(p);

    while let Some(i) = q.pop_front() {
        let mi = (i - k + 1).max(k - i - 1);
        let mx = (i + k - 1).min(2 * n - k - i - 1);
        let s = &mut ts[(mi % 2) as usize];

        while let Some(&j) = s.range(mi..=mx).next() {
            q.push_back(j);
            ans[j as usize] = ans[i as usize] + 1;
            s.remove(&j);
        }
    }
    ans
}

fn main() {
    println!("{:?}", min_reverse_operations(4, 0, vec![1, 2], 4));
}

#[cfg(test)]
mod tests {
    use super::min_reverse_operations;

    #[test]
    fn example_one() {
        assert_eq!(
            min_reverse_operations(4, 0, vec![1, 2], 4),
            vec![0, -1, -1, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_reverse_operations(5, 0, vec![2, 4], 3),
            vec![0, -1, -1, -1, -1]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            min_reverse_operations(4, 2, vec![0, 1, 3], 1),
            vec![-1, -1, 0, -1]
        );
    }
}
