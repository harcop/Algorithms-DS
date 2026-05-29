/// LeetCode #1500 - Design A File Sharing System
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct FileSharing {
    chunks: i32,
    cur: i32,
    reused: BinaryHeap<Reverse<i32>>,
    user_chunks: HashMap<i32, HashSet<i32>>,
}

impl FileSharing {
    fn new(m: i32) -> Self {
        FileSharing { chunks: m, cur: 0, reused: BinaryHeap::new(), user_chunks: HashMap::new() }
    }
    fn join(&mut self, owned_chunks: Vec<i32>) -> i32 {
        let user_id = if let Some(Reverse(id)) = self.reused.pop() { id } else { self.cur += 1; self.cur };
        self.user_chunks.insert(user_id, owned_chunks.into_iter().collect());
        user_id
    }
    fn leave(&mut self, user_id: i32) {
        if self.user_chunks.remove(&user_id).is_some() { self.reused.push(Reverse(user_id)); }
    }
    fn request(&mut self, user_id: i32, chunk_id: i32) -> Vec<i32> {
        if chunk_id < 1 || chunk_id > self.chunks { return vec![]; }
        let mut owners: Vec<i32> = self.user_chunks.iter()
            .filter(|(&uid, chunks)| uid != user_id && chunks.contains(&chunk_id))
            .map(|(&uid, _)| uid).collect();
        if !owners.is_empty() {
            self.user_chunks.entry(user_id).or_default().insert(chunk_id);
            owners.sort_unstable();
        }
        owners
    }
}
fn main() { let mut fs = FileSharing::new(4); println!("{}", fs.join(vec![1, 2])); }
#[cfg(test)]
mod tests {
    use super::FileSharing;
    #[test]
    fn example_one() {
        let mut fs = FileSharing::new(4);
        assert_eq!(fs.join(vec![1, 2]), 1);
        assert_eq!(fs.join(vec![2, 3]), 2);
        assert_eq!(fs.join(vec![4]), 3);
        assert_eq!(fs.request(1, 3), vec![2]);
        assert_eq!(fs.request(2, 2), vec![1]);
        fs.leave(1);
        assert_eq!(fs.join(vec![1, 3]), 1);
        assert_eq!(fs.request(1, 1), Vec::<i32>::new());
        assert_eq!(fs.request(2, 1), vec![1]);
    }
}
