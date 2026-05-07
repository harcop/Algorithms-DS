/// LeetCode #352 - Data Stream as Disjoint Intervals
use std::collections::BTreeMap;

struct SummaryRanges {
    /// start -> end (inclusive)
    mp: BTreeMap<i32, i32>,
}

impl SummaryRanges {
    fn new() -> Self {
        SummaryRanges { mp: BTreeMap::new() }
    }

    fn add_num(&mut self, value: i32) {
        if let Some((&_, &hi)) = self.mp.range(..=value).next_back() {
            if value <= hi {
                return;
            }
        }
        let mut start = value;
        let mut end = value;
        if let Some((&lo, &hi)) = self.mp.range(..=value).next_back() {
            if hi + 1 >= value {
                start = lo;
                self.mp.remove(&lo);
                end = end.max(hi);
            }
        }
        let mut to_remove: Vec<i32> = vec![];
        for (&s, &e) in self.mp.range(value..) {
            if s > end + 1 {
                break;
            }
            start = start.min(s);
            end = end.max(e);
            to_remove.push(s);
        }
        for s in to_remove {
            self.mp.remove(&s);
        }
        self.mp.insert(start, end);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.mp.iter().map(|(&s, &e)| vec![s, e]).collect()
    }
}

fn main() {
    let mut s = SummaryRanges::new();
    s.add_num(1);
    s.add_num(3);
    s.add_num(7);
    s.add_num(2);
    s.add_num(6);
    println!("{:?}", s.get_intervals());
}

#[cfg(test)]
mod tests {
    use super::SummaryRanges;

    #[test]
    fn merges() {
        let mut s = SummaryRanges::new();
        s.add_num(1);
        s.add_num(3);
        s.add_num(7);
        s.add_num(2);
        s.add_num(6);
        assert_eq!(s.get_intervals(), vec![vec![1, 3], vec![6, 7]]);
    }
}
