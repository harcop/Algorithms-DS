/// LeetCode #2013 - Detect Squares
use std::collections::HashMap;

pub struct DetectSquares {
    cnt: HashMap<i32, HashMap<i32, i32>>,
}

impl DetectSquares {
    fn new() -> Self {
        DetectSquares {
            cnt: HashMap::new(),
        }
    }

    fn add(&mut self, point: Vec<i32>) {
        let (x, y) = (point[0], point[1]);
        *self.cnt.entry(x).or_default().entry(y).or_insert(0) += 1;
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let (x1, y1) = (point[0], point[1]);
        let Some(row) = self.cnt.get(&x1) else {
            return 0;
        };
        let mut ans = 0;
        for (&x2, col) in &self.cnt {
            if x2 == x1 {
                continue;
            }
            let d = x2 - x1;
            ans += col.get(&y1).copied().unwrap_or(0)
                * row.get(&(y1 + d)).copied().unwrap_or(0)
                * col.get(&(y1 + d)).copied().unwrap_or(0);
            ans += col.get(&y1).copied().unwrap_or(0)
                * row.get(&(y1 - d)).copied().unwrap_or(0)
                * col.get(&(y1 - d)).copied().unwrap_or(0);
        }
        ans
    }
}

fn main() {
    let mut ds = DetectSquares::new();
    ds.add(vec![3, 10]);
    ds.add(vec![11, 2]);
    ds.add(vec![3, 2]);
    println!("{}", ds.count(vec![11, 10]));
}

#[cfg(test)]
mod tests {
    use super::DetectSquares;

    #[test]
    fn example_one() {
        let mut ds = DetectSquares::new();
        ds.add(vec![3, 10]);
        ds.add(vec![11, 2]);
        ds.add(vec![3, 2]);
        assert_eq!(ds.count(vec![11, 10]), 1);
        ds.add(vec![3, 2]);
        assert_eq!(ds.count(vec![11, 10]), 2);
    }
}
