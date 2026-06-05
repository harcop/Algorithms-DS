/// LeetCode #1756 - Design Most Recently Used Queue
pub struct MRUQueue {
    data: Vec<i32>,
}

impl MRUQueue {
    fn new(n: i32) -> Self {
        Self {
            data: (1..=n).collect(),
        }
    }
    fn fetch(&mut self, k: i32) -> i32 {
        let idx = (k - 1) as usize;
        let v = self.data[idx];
        self.data.remove(idx);
        self.data.push(v);
        v
    }
}

fn run_ops(n: i32, fetch: Vec<i32>) -> Vec<i32> {
    let mut q = MRUQueue::new(n);
    fetch.into_iter().map(|k| q.fetch(k)).collect()
}
fn main() { println!("{:?}", run_ops(8, vec![3, 5, 2, 8])); }
#[cfg(test)]
mod tests {
    use super::run_ops;
    #[test]
    fn example_one() {
        assert_eq!(run_ops(8, vec![3, 5, 2, 8]), vec![3, 6, 2, 2]);
    }
    #[test]
    fn example_two() {
        assert_eq!(run_ops(5, vec![3, 1, 1, 1, 1]), vec![3, 1, 2, 4, 5]);
    }
}
