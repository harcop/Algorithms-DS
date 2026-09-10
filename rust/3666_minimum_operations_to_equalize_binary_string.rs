/// LeetCode #3666 - Minimum Operations to Equalize Binary String
use std::collections::{BTreeSet, VecDeque};

fn min_operations(s: String, k: i32) -> i32 {
    let n = s.len() as i32;
    let mut ts = [BTreeSet::new(), BTreeSet::new()];
    for i in 0..=n {
        ts[(i % 2) as usize].insert(i);
    }
    let cnt0 = s.bytes().filter(|&c| c == b'0').count() as i32;
    ts[(cnt0 % 2) as usize].remove(&cnt0);
    let mut q = VecDeque::new();
    q.push_back(cnt0);
    let mut ans = 0;
    while !q.is_empty() {
        let size = q.len();
        for _ in 0..size {
            let cur = q.pop_front().unwrap();
            if cur == 0 {
                return ans;
            }
            let l = cur + k - 2 * cur.min(k);
            let r = cur + k - 2 * (k - n + cur).max(0);
            let parity = (l % 2) as usize;
            let vals: Vec<i32> = ts[parity].range(l..=r).cloned().collect();
            for v in vals {
                q.push_back(v);
                ts[parity].remove(&v);
            }
        }
        ans += 1;
    }
    -1
}

fn main() {
    println!("{}", min_operations("110".into(), 1));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations("110".into(), 1), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations("0101".into(), 3), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations("101".into(), 2), -1);
    }
}
