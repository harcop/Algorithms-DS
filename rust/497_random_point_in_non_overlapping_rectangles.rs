/// LeetCode #497 - Random Point in Non-overlapping Rectangles
struct Solution {
    rects: Vec<Vec<i32>>,
    prefix: Vec<i64>,
    rng: u64,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut prefix = Vec::new();
        let mut acc = 0i64;
        for r in &rects {
            let w = (r[2] - r[0] + 1) as i64;
            let h = (r[3] - r[1] + 1) as i64;
            acc += w * h;
            prefix.push(acc);
        }
        Self {
            rects,
            prefix,
            rng: 0x9e37_79b9_7f4a_7c15,
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.rng ^= self.rng << 7;
        self.rng ^= self.rng >> 9;
        self.rng
    }

    fn pick(&mut self) -> Vec<i32> {
        let total = *self.prefix.last().unwrap();
        let r = (self.next_u64() % total as u64) as i64 + 1;
        let idx = self.prefix.partition_point(|&x| x < r);
        let (x1, y1, x2, y2) = (
            self.rects[idx][0],
            self.rects[idx][1],
            self.rects[idx][2],
            self.rects[idx][3],
        );
        let w = (x2 - x1 + 1) as u64;
        let h = (y2 - y1 + 1) as u64;
        let x = x1 + (self.next_u64() % w) as i32;
        let y = y1 + (self.next_u64() % h) as i32;
        vec![x, y]
    }
}

fn main() {
    let mut s = Solution::new(vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]]);
    println!("{:?}", s.pick());
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn in_rects(p: &[i32], rects: &[Vec<i32>]) -> bool {
        rects.iter().any(|r| p[0] >= r[0] && p[0] <= r[2] && p[1] >= r[1] && p[1] <= r[3])
    }

    #[test]
    fn example_one() {
        let rects = vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]];
        let mut s = Solution::new(rects.clone());
        for _ in 0..50 {
            let p = s.pick();
            assert!(in_rects(&p, &rects), "{:?}", p);
        }
    }

    #[test]
    fn single_rect() {
        let rects = vec![vec![1, 1, 5, 5]];
        let mut s = Solution::new(rects.clone());
        for _ in 0..20 {
            let p = s.pick();
            assert!(in_rects(&p, &rects), "{:?}", p);
        }
    }
}
