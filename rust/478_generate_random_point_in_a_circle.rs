/// LeetCode #478 - Generate Random Point in a Circle
struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
    rng: u64,
}

impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            x_center,
            y_center,
            rng: 0x1234_5678_9abc_def0,
        }
    }

    fn next_unit(&mut self) -> f64 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 7;
        self.rng ^= self.rng << 17;
        (self.rng as f64) / (u64::MAX as f64)
    }

    fn rand_point(&mut self) -> Vec<f64> {
        loop {
            let x = self.next_unit() * 2.0 - 1.0;
            let y = self.next_unit() * 2.0 - 1.0;
            if x * x + y * y <= 1.0 {
                return vec![
                    self.x_center + x * self.radius,
                    self.y_center + y * self.radius,
                ];
            }
        }
    }
}

fn main() {
    let mut s = Solution::new(1.0, 0.0, 0.0);
    println!("{:?}", s.rand_point());
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn inside(p: &[f64], r: f64, x: f64, y: f64) -> bool {
        let dx = p[0] - x;
        let dy = p[1] - y;
        dx * dx + dy * dy <= r * r + 1e-9
    }

    #[test]
    fn example_unit_circle() {
        let mut s = Solution::new(1.0, 0.0, 0.0);
        for _ in 0..50 {
            let p = s.rand_point();
            assert!(inside(&p, 1.0, 0.0, 0.0), "{:?}", p);
        }
    }

    #[test]
    fn example_offset() {
        let mut s = Solution::new(10.0, 5.0, -7.5);
        for _ in 0..50 {
            let p = s.rand_point();
            assert!(inside(&p, 10.0, 5.0, -7.5), "{:?}", p);
        }
    }
}
