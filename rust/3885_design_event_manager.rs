/// LeetCode #3885 - Design Event Manager
use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap};

struct EventManager {
    priority: HashMap<i32, i32>,
    heap: BTreeSet<(Reverse<i32>, i32)>,
}

impl EventManager {
    fn new(events: Vec<Vec<i32>>) -> Self {
        let mut em = EventManager {
            priority: HashMap::new(),
            heap: BTreeSet::new(),
        };
        for e in events {
            em.insert(e[0], e[1]);
        }
        em
    }

    fn insert(&mut self, id: i32, p: i32) {
        self.priority.insert(id, p);
        self.heap.insert((Reverse(p), id));
    }

    fn update_priority(&mut self, id: i32, new_priority: i32) {
        let old = self.priority[&id];
        self.heap.remove(&(Reverse(old), id));
        self.priority.insert(id, new_priority);
        self.heap.insert((Reverse(new_priority), id));
    }

    fn poll_highest(&mut self) -> i32 {
        let Some(&(Reverse(_), id)) = self.heap.iter().next() else {
            return -1;
        };
        self.heap.remove(&(Reverse(self.priority[&id]), id));
        self.priority.remove(&id);
        id
    }
}

fn main() {
    let mut em = EventManager::new(vec![vec![5, 7], vec![2, 7], vec![9, 4]]);
    println!("{}", em.poll_highest());
}

#[cfg(test)]
mod tests {
    use super::EventManager;

    #[test]
    fn example1() {
        let mut em = EventManager::new(vec![vec![5, 7], vec![2, 7], vec![9, 4]]);
        assert_eq!(em.poll_highest(), 2);
        em.update_priority(9, 7);
        assert_eq!(em.poll_highest(), 5);
        assert_eq!(em.poll_highest(), 9);
    }

    #[test]
    fn example2() {
        let mut em = EventManager::new(vec![vec![4, 1], vec![7, 2]]);
        assert_eq!(em.poll_highest(), 7);
        assert_eq!(em.poll_highest(), 4);
        assert_eq!(em.poll_highest(), -1);
    }
}
