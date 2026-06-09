/// LeetCode #1792 - Maximum Average Pass Ratio
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct ClassEntry {
    a: i32,
    b: i32,
}

impl Ord for ClassEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        let d1 = self.b as i64 * (self.b as i64 + 1);
        let d2 = other.b as i64 * (other.b as i64 + 1);
        let n1 = (self.a - self.b) as i64;
        let n2 = (other.a - other.b) as i64;
        (n1 * d2).cmp(&(n2 * d1))
    }
}

impl PartialOrd for ClassEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
    let mut h = BinaryHeap::new();
    for c in &classes {
        h.push(Reverse(ClassEntry { a: c[0], b: c[1] }));
    }
    for _ in 0..extra_students {
        let Reverse(mut entry) = h.pop().unwrap();
        entry.a += 1;
        entry.b += 1;
        h.push(Reverse(entry));
    }
    let sum: f64 = h
        .iter()
        .map(|Reverse(e)| e.a as f64 / e.b as f64)
        .sum();
    sum / classes.len() as f64
}

fn main() {
    println!(
        "{}",
        max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::max_average_ratio;

    #[test]
    fn example_one() {
        let v = max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2);
        assert!((v - 0.78333).abs() < 1e-4);
    }

    #[test]
    fn example_two() {
        let v = max_average_ratio(vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]], 4);
        assert!((v - 0.53485).abs() < 1e-4);
    }
}
