/// LeetCode #1538 - Guess The Majority In A Hidden Array
use std::collections::HashMap;

pub struct MajorityChecker {
    arr: Vec<i32>,
    pos: HashMap<i32, Vec<usize>>,
}

impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let mut pos = HashMap::new();
        for (i, &v) in arr.iter().enumerate() {
            pos.entry(v).or_insert_with(Vec::new).push(i);
        }
        MajorityChecker { arr, pos }
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let l = left as usize;
        let r = right as usize;
        let mut ans = -1;
        for (&val, _idxs) in &self.pos {
            let cnt = self.arr[l..=r].iter().filter(|&&x| x == val).count();
            if cnt >= threshold as usize && (ans == -1 || val < ans) {
                ans = val;
            }
        }
        ans
    }
}

fn main() {
    let mc = MajorityChecker::new(vec![2, 1, 1, 2, 1, 2, 2, 1, 1, 2]);
    println!("{}", mc.query(0, 9, 5));
}

#[cfg(test)]
mod tests {
    use super::MajorityChecker;

    #[test]
    fn example_one() {
        let mc = MajorityChecker::new(vec![2, 1, 1, 2, 1, 2, 2, 1, 1, 2]);
        assert_eq!(mc.query(1, 9, 5), 1);
        assert_eq!(mc.query(0, 5, 4), -1);
        assert_eq!(mc.query(0, 9, 5), 1);
        assert_eq!(mc.query(0, 9, 6), -1);
    }
}
