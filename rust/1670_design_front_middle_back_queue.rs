/// LeetCode #1670 - Design Front Middle Back Queue
pub struct FrontMiddleBackQueue {
    data: Vec<i32>,
}

impl FrontMiddleBackQueue {
    fn new() -> Self { Self { data: vec![] } }
    fn push_front(&mut self, val: i32) { self.data.insert(0, val); }
    fn push_middle(&mut self, val: i32) { self.data.insert(self.data.len() / 2, val); }
    fn push_back(&mut self, val: i32) { self.data.push(val); }
    fn pop_front(&mut self) -> i32 { if self.data.is_empty() { -1 } else { self.data.remove(0) } }
    fn pop_middle(&mut self) -> i32 {
        if self.data.is_empty() { -1 } else { self.data.remove((self.data.len() - 1) / 2) }
    }
    fn pop_back(&mut self) -> i32 { self.data.pop().unwrap_or(-1) }
}
fn main() {
    let mut q = FrontMiddleBackQueue::new();
    q.push_front(1);
    println!("{}", q.pop_back());
}
#[cfg(test)]
mod tests {
    use super::FrontMiddleBackQueue;
    #[test]
    fn example_one() {
        let mut q = FrontMiddleBackQueue::new();
        q.push_front(1); q.push_back(2); q.push_middle(3); q.push_middle(4);
        assert_eq!(q.pop_front(), 1);
        assert_eq!(q.pop_middle(), 3);
        assert_eq!(q.pop_middle(), 4);
        assert_eq!(q.pop_back(), 2);
        assert_eq!(q.pop_front(), -1);
    }
}