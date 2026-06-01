/// LeetCode #1656 - Design An Ordered Stream
pub struct OrderedStream {
    n: i32,
    ptr: i32,
    stream: Vec<String>,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        OrderedStream { n, ptr: 1, stream: vec![String::new(); n as usize + 1] }
    }
    fn insert(&mut self, id: i32, value: String) -> Vec<String> {
        self.stream[id as usize] = value;
        let mut ans = vec![];
        while self.ptr <= self.n && !self.stream[self.ptr as usize].is_empty() {
            ans.push(self.stream[self.ptr as usize].clone());
            self.ptr += 1;
        }
        ans
    }
}
fn main() {
    let mut os = OrderedStream::new(5);
    println!("{:?}", os.insert(3, "ccccc".into()));
}
#[cfg(test)]
mod tests {
    use super::OrderedStream;
    #[test]
    fn example_one() {
        let mut os = OrderedStream::new(5);
        assert_eq!(os.insert(3, "ccccc".into()), Vec::<String>::new());
        assert_eq!(os.insert(1, "aaaaa".into()), vec!["aaaaa"]);
        assert_eq!(os.insert(2, "bbbbb".into()), vec!["bbbbb", "ccccc"]);
    }
}