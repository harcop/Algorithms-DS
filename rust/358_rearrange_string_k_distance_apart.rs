/// LeetCode #358 - Rearrange String k Distance Apart
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

#[derive(Eq, PartialEq)]
struct Node {
    c: usize,
    ch: u8,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.c.cmp(&other.c)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn rearrange_string(s: String, k: i32) -> String {
    let k = k as usize;
    if k <= 1 {
        return s;
    }
    let mut f = [0usize; 26];
    for b in s.bytes() {
        f[(b - b'a') as usize] += 1;
    }
    let mut heap = BinaryHeap::new();
    for i in 0..26 {
        if f[i] > 0 {
            heap.push(Node {
                c: f[i],
                ch: i as u8 + b'a',
            });
        }
    }
    let mut wait: VecDeque<(u8, usize)> = VecDeque::new();
    let mut out: Vec<u8> = Vec::with_capacity(s.len());
    for _ in 0..s.len() {
        if heap.is_empty() {
            return "".into();
        }
        let mut cur = heap.pop().unwrap();
        out.push(cur.ch);
        cur.c -= 1;
        wait.push_back((cur.ch, cur.c));
        if wait.len() == k - 1 {
            let (ch, rem) = wait.pop_front().unwrap();
            if rem > 0 {
                heap.push(Node { c: rem, ch });
            }
        }
    }
    unsafe { String::from_utf8_unchecked(out) }
}

fn main() {
    println!("{}", rearrange_string("aabbcc".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid(s: &str, k: usize) -> bool {
        if k <= 1 {
            return true;
        }
        let b = s.as_bytes();
        let mut last = [None; 26];
        for (i, &ch) in b.iter().enumerate() {
            let ix = (ch - b'a') as usize;
            if let Some(p) = last[ix] {
                if i - p < k {
                    return false;
                }
            }
            last[ix] = Some(i);
        }
        true
    }

    #[test]
    fn examples() {
        let r = rearrange_string("aabbcc".into(), 3);
        assert_eq!(r.len(), 6);
        assert!(valid(&r, 3));
        assert_eq!(rearrange_string("aaadbbcc".into(), 2).len(), 8);
    }
}
