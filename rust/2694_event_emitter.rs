/// LeetCode #2694 - Event Emitter (JS problem; Rust struct analogue)
use std::collections::HashMap;

struct EventEmitter {
    listeners: HashMap<String, Vec<(usize, Box<dyn Fn(&[i32]) -> i32>)>>,
    next_id: usize,
}

impl EventEmitter {
    fn new() -> Self {
        EventEmitter {
            listeners: HashMap::new(),
            next_id: 0,
        }
    }

    fn subscribe<F>(&mut self, event: &str, callback: F) -> usize
    where
        F: Fn(&[i32]) -> i32 + 'static,
    {
        let id = self.next_id;
        self.next_id += 1;
        self.listeners
            .entry(event.to_string())
            .or_default()
            .push((id, Box::new(callback)));
        id
    }

    fn unsubscribe(&mut self, event: &str, id: usize) {
        if let Some(list) = self.listeners.get_mut(event) {
            list.retain(|(sid, _)| *sid != id);
        }
    }

    fn emit(&self, event: &str, args: &[i32]) -> Vec<i32> {
        match self.listeners.get(event) {
            None => vec![],
            Some(list) => list.iter().map(|(_, cb)| cb(args)).collect(),
        }
    }
}

fn main() {
    let mut emitter = EventEmitter::new();
    emitter.subscribe("firstEvent", |_| 5);
    emitter.subscribe("firstEvent", |_| 6);
    println!("{:?}", emitter.emit("firstEvent", &[]));
}

#[cfg(test)]
mod tests {
    use super::EventEmitter;

    #[test]
    fn example_one() {
        let mut emitter = EventEmitter::new();
        assert_eq!(emitter.emit("firstEvent", &[]), vec![]);
        emitter.subscribe("firstEvent", |_| 5);
        emitter.subscribe("firstEvent", |_| 6);
        assert_eq!(emitter.emit("firstEvent", &[]), vec![5, 6]);
    }

    #[test]
    fn example_two() {
        let mut emitter = EventEmitter::new();
        emitter.subscribe("firstEvent", |args| args.iter().sum());
        assert_eq!(emitter.emit("firstEvent", &[1, 2, 3]), vec![6]);
        assert_eq!(emitter.emit("firstEvent", &[3, 4, 6]), vec![13]);
    }

    #[test]
    fn example_unsubscribe() {
        let mut emitter = EventEmitter::new();
        let id1 = emitter.subscribe("firstEvent", |args| args[0] + 1);
        emitter.subscribe("firstEvent", |args| args[0] + 2);
        emitter.unsubscribe("firstEvent", id1);
        assert_eq!(emitter.emit("firstEvent", &[5]), vec![7]);
    }
}
