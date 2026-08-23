/// LeetCode #3382 - Maximum Area Rectangle With Point Constraints II
use std::collections::HashMap;

struct Bit {
    bit: Vec<i32>,
}

impl Bit {
    fn new(n: usize) -> Self {
        Bit {
            bit: vec![0; n + 2],
        }
    }
    fn add(&mut self, mut i: i32, val: i32) {
        i += 1;
        while (i as usize) < self.bit.len() {
            self.bit[i as usize] += val;
            i += i & -i;
        }
    }
    fn query(&self, mut i: i32) -> i32 {
        if i < 0 {
            return 0;
        }
        i += 1;
        let mut total = 0;
        while i > 0 {
            total += self.bit[i as usize];
            i -= i & -i;
        }
        total
    }
}

fn max_rectangle_area(x_coord: Vec<i32>, y_coord: Vec<i32>) -> i64 {
    let mut points: Vec<(i32, i32)> = x_coord.into_iter().zip(y_coord).collect();
    points.sort_unstable();
    let mut ys: Vec<i32> = points.iter().map(|&(_, y)| y).collect();
    ys.sort_unstable();
    ys.dedup();
    let y_to_idx: HashMap<i32, usize> = ys.iter().enumerate().map(|(i, &y)| (y, i)).collect();
    let mut bit = Bit::new(ys.len());
    let mut lookup: HashMap<(usize, usize), (i32, i32)> = HashMap::new();
    let mut result = -1i64;
    for i in 0..points.len() {
        let (x, y) = points[i];
        let y_idx = y_to_idx[&y];
        bit.add(y_idx as i32, 1);
        if i == 0 || points[i - 1].0 != x {
            continue;
        }
        let prev_y = points[i - 1].1;
        let prev_y_idx = y_to_idx[&prev_y];
        let curr = bit.query(y_idx as i32) - bit.query(prev_y_idx as i32 - 1);
        if let Some(&(prev, prev_x)) = lookup.get(&(prev_y_idx, y_idx)) {
            if prev == curr - 2 {
                result = result.max((x as i64 - prev_x as i64) * (y as i64 - prev_y as i64));
            }
        }
        lookup.insert((prev_y_idx, y_idx), (curr, x));
    }
    result
}

fn main() {
    println!("{}", max_rectangle_area(vec![1, 1, 3, 3], vec![1, 3, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::max_rectangle_area;

    #[test]
    fn example1() {
        assert_eq!(max_rectangle_area(vec![1, 1, 3, 3], vec![1, 3, 1, 3]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_rectangle_area(vec![1, 1, 3, 3, 2], vec![1, 3, 1, 3, 2]),
            -1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            max_rectangle_area(vec![1, 1, 3, 3, 1, 3], vec![1, 3, 1, 3, 2, 2]),
            2
        );
    }
}
