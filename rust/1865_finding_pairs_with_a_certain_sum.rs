/// LeetCode #1865 - Finding Pairs With a Certain Sum
use std::collections::HashMap;

pub struct FindSumPairs {
    cnt: HashMap<i32, i32>,
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}

impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut cnt = HashMap::new();
        for &x in &nums2 {
            *cnt.entry(x).or_insert(0) += 1;
        }
        FindSumPairs { cnt, nums1, nums2 }
    }

    fn add(&mut self, index: i32, val: i32) {
        let idx = index as usize;
        *self.cnt.get_mut(&self.nums2[idx]).unwrap() -= 1;
        self.nums2[idx] += val;
        *self.cnt.entry(self.nums2[idx]).or_insert(0) += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        self.nums1
            .iter()
            .map(|&x| self.cnt.get(&(tot - x)).copied().unwrap_or(0))
            .sum()
    }
}

fn main() {
    let obj = FindSumPairs::new(vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]);
    println!("{}", obj.count(7));
}

#[cfg(test)]
mod tests {
    use super::FindSumPairs;

    #[test]
    fn example_one() {
        let mut obj = FindSumPairs::new(vec![1, 1, 2, 2, 2, 3], vec![1, 4, 5, 2, 5, 4]);
        assert_eq!(obj.count(7), 8);
        obj.add(3, 2);
        assert_eq!(obj.count(8), 2);
        assert_eq!(obj.count(4), 1);
        obj.add(0, 1);
        obj.add(1, 1);
        assert_eq!(obj.count(7), 11);
    }
}
