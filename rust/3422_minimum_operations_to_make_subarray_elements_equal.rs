/// LeetCode #3422 - Minimum Operations to Make Subarray Elements Equal
use std::collections::BTreeMap;

struct MultiSet {
    m: BTreeMap<i64, usize>,
    len: usize,
}

impl MultiSet {
    fn new() -> Self {
        Self {
            m: BTreeMap::new(),
            len: 0,
        }
    }
    fn insert(&mut self, x: i64) {
        *self.m.entry(x).or_insert(0) += 1;
        self.len += 1;
    }
    fn remove(&mut self, x: i64) -> bool {
        if let Some(c) = self.m.get_mut(&x) {
            *c -= 1;
            self.len -= 1;
            if *c == 0 {
                self.m.remove(&x);
            }
            true
        } else {
            false
        }
    }
    fn contains(&self, x: i64) -> bool {
        self.m.contains_key(&x)
    }
    fn min(&self) -> i64 {
        *self.m.keys().next().unwrap()
    }
    fn max(&self) -> i64 {
        *self.m.keys().next_back().unwrap()
    }
}

fn min_operations(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut l = MultiSet::new();
    let mut r = MultiSet::new();
    let mut s1 = 0i64;
    let mut s2 = 0i64;
    let mut ans = i64::MAX;
    for (i, &x) in nums.iter().enumerate() {
        let x = x as i64;
        l.insert(x);
        s1 += x;
        let y = l.max();
        l.remove(y);
        s1 -= y;
        r.insert(y);
        s2 += y;
        if r.len as i32 - l.len as i32 > 1 {
            let y = r.min();
            r.remove(y);
            s2 -= y;
            l.insert(y);
            s1 += y;
        }
        if i + 1 >= k {
            let med = r.min();
            ans = ans.min(s2 - med * r.len as i64 + med * l.len as i64 - s1);
            let j = nums[i + 1 - k] as i64;
            if r.contains(j) {
                r.remove(j);
                s2 -= j;
            } else {
                l.remove(j);
                s1 -= j;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", min_operations(vec![4, -3, 2, 1, -4, 6], 3));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![4, -3, 2, 1, -4, 6], 3), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![-2, -2, 3, 1, 4], 2), 0);
    }
}
