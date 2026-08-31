/// LeetCode #3508 - Implement Router
use std::collections::{HashMap, HashSet, VecDeque};

struct Router {
    lim: usize,
    vis: HashSet<(i32, i32, i32)>,
    q: VecDeque<(i32, i32, i32)>,
    idx: HashMap<i32, usize>,
    d: HashMap<i32, Vec<i32>>,
}

impl Router {
    fn new(memory_limit: i32) -> Self {
        Self {
            lim: memory_limit as usize,
            vis: HashSet::new(),
            q: VecDeque::new(),
            idx: HashMap::new(),
            d: HashMap::new(),
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let x = (source, destination, timestamp);
        if self.vis.contains(&x) {
            return false;
        }
        self.vis.insert(x);
        if self.q.len() >= self.lim {
            self.forward_packet();
        }
        self.q.push_back(x);
        self.d.entry(destination).or_default().push(timestamp);
        true
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        let Some((s, d, t)) = self.q.pop_front() else {
            return vec![];
        };
        self.vis.remove(&(s, d, t));
        *self.idx.entry(d).or_insert(0) += 1;
        vec![s, d, t]
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        let Some(ls) = self.d.get(&destination) else {
            return 0;
        };
        let k = *self.idx.get(&destination).unwrap_or(&0);
        if k >= ls.len() {
            return 0;
        }
        let slice = &ls[k..];
        let i = slice.partition_point(|&t| t < start_time);
        let j = slice.partition_point(|&t| t <= end_time);
        (j - i) as i32
    }
}

fn main() {
    let mut router = Router::new(3);
    println!("{}", router.add_packet(1, 4, 90));
}

#[cfg(test)]
mod tests {
    use super::Router;

    #[test]
    fn example1() {
        let mut router = Router::new(3);
        assert!(router.add_packet(1, 4, 90));
        assert!(router.add_packet(2, 5, 90));
        assert!(!router.add_packet(1, 4, 90));
        assert!(router.add_packet(3, 5, 95));
        assert!(router.add_packet(4, 5, 105));
        assert_eq!(router.forward_packet(), vec![2, 5, 90]);
        assert!(router.add_packet(5, 2, 110));
        assert_eq!(router.get_count(5, 100, 110), 1);
    }

    #[test]
    fn example2() {
        let mut router = Router::new(2);
        assert!(router.add_packet(7, 4, 90));
        assert_eq!(router.forward_packet(), vec![7, 4, 90]);
        assert_eq!(router.forward_packet(), Vec::<i32>::new());
    }
}
