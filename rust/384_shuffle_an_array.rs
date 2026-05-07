/// LeetCode #384 - Shuffle an Array (Fisher–Yates)
struct Solution { orig: Vec<i32>, cur: Vec<i32> }

impl Solution {
    fn new(nums: Vec<i32>) -> Self { Solution { orig: nums.clone(), cur: nums } }
    fn reset(&mut self) -> Vec<i32> { self.cur.clone_from(&self.orig); self.cur.clone() }
    fn shuffle(&mut self) -> Vec<i32> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let n = self.cur.len();
        for i in (1..n).rev() {
            let j = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as usize) % (i + 1);
            self.cur.swap(i, j);
        }
        self.cur.clone()
    }
}

fn main() { let mut s=Solution::new(vec![1,2,3]); println!("{:?}", s.shuffle()); }

#[cfg(test)] mod tests { use super::*; #[test] fn sm(){
    let mut s=Solution::new(vec![1,2,3]); let r=s.reset(); assert_eq!(r.len(),3);
}}
