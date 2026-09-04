/// LeetCode #1157 - Online Majority Element In Subarray
use std::collections::HashMap;

struct MajorityChecker {
    pos: HashMap<i32, Vec<usize>>,
}

impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let mut pos = HashMap::new();
        for (i, v) in arr.into_iter().enumerate() {
            pos.entry(v).or_insert_with(Vec::new).push(i);
        }
        MajorityChecker { pos }
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        for (&val, idxs) in &self.pos {
            let lo = idxs.partition_point(|&p| p < left);
            let hi = idxs.partition_point(|&p| p <= right);
            if (hi - lo) as i32 >= threshold {
                return val;
            }
        }
        -1
    }
}

fn main() {
    let mc = MajorityChecker::new(vec![1, 1, 2, 2, 1, 1]);
    println!("{}", mc.query(0, 5, 4));
}

#[cfg(test)]
mod tests {
    use super::MajorityChecker;

    #[test]
    fn example() {
        let mc = MajorityChecker::new(vec![1, 1, 2, 2, 1, 1]);
        assert_eq!(mc.query(0, 5, 4), 1);
        assert_eq!(mc.query(0, 3, 3), -1);
        assert_eq!(mc.query(2, 3, 2), 2);
    }
}
