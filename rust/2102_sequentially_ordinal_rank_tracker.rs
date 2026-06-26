/// LeetCode #2102 - Sequentially Ordinal Rank Tracker
use std::cmp::Ordering;
use std::collections::BTreeSet;

#[derive(Clone, Eq, PartialEq)]
struct Location {
    score: i32,
    name: String,
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score).then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct SORTracker {
    top: BTreeSet<Location>,
    rest: BTreeSet<Location>,
}

impl SORTracker {
    fn new() -> Self {
        SORTracker {
            top: BTreeSet::new(),
            rest: BTreeSet::new(),
        }
    }

    fn add(&mut self, name: String, score: i32) {
        self.top.insert(Location { score, name });
        if let Some(worst) = self.top.iter().next_back().cloned() {
            self.top.remove(&worst);
            self.rest.insert(worst);
        }
    }

    fn get(&mut self) -> String {
        let best_rest = self.rest.iter().next().cloned().unwrap();
        self.rest.remove(&best_rest);
        self.top.insert(best_rest);
        self.top.iter().next_back().unwrap().name.clone()
    }
}

fn main() {
    let mut tracker = SORTracker::new();
    tracker.add("bradford".into(), 2);
    tracker.add("branford".into(), 3);
    println!("{}", tracker.get());
}

#[cfg(test)]
mod tests {
    use super::SORTracker;

    #[test]
    fn example_one() {
        let mut tracker = SORTracker::new();
        tracker.add("bradford".into(), 2);
        tracker.add("branford".into(), 3);
        assert_eq!(tracker.get(), "branford");
        tracker.add("alps".into(), 2);
        assert_eq!(tracker.get(), "alps");
        tracker.add("orland".into(), 2);
        assert_eq!(tracker.get(), "bradford");
        tracker.add("orlando".into(), 3);
        assert_eq!(tracker.get(), "bradford");
        tracker.add("alpine".into(), 2);
        assert_eq!(tracker.get(), "bradford");
        assert_eq!(tracker.get(), "orland");
    }
}
