/// LeetCode #382 - Linked List Random Node (collect values; uniform pick)
#[derive(Clone, Debug)] pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

struct Solution { vals: Vec<i32>, }

impl Solution {
    fn new(mut head: Option<Box<ListNode>>) -> Self {
        let mut vals = vec![];
        while let Some(n) = head { vals.push(n.val); head=n.next; }
        Solution { vals }
    }

    fn get_random(&self) -> i32 {
        let len = self.vals.len().max(1);
        let i = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos() as usize % len;
        self.vals[i]
    }
}

fn main() {
    println!("{}", Solution::new(None).vals.len());
}

#[cfg(test)] mod tests { use super::*;
    #[test] fn rr() {
        let mut h=None;
        for &x in [1,2,3].iter().rev() {
            h=Some(Box::new(ListNode{val:x,next:h}));
        }
        let s=Solution::new(h);
        let _ = s.get_random();
    }
}
