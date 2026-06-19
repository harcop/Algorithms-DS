/// LeetCode #1964 - Find the Longest Valid Obstacle Course at Each Position
struct Fenwick {
    n: usize,
    bit: Vec<i32>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self {
            n,
            bit: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut x: usize, v: i32) {
        while x <= self.n {
            self.bit[x] = self.bit[x].max(v);
            x += x & x.wrapping_neg();
        }
    }

    fn query(&self, mut x: usize) -> i32 {
        let mut s = 0i32;
        while x > 0 {
            s = s.max(self.bit[x]);
            x -= x & x.wrapping_neg();
        }
        s
    }
}

fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
    let mut nums: Vec<i32> = obstacles.iter().copied().collect();
    nums.sort_unstable();
    nums.dedup();
    let n = nums.len();
    let mut tree = Fenwick::new(n);
    let mut ans = Vec::with_capacity(obstacles.len());

    for x in obstacles {
        let i = nums.partition_point(|&v| v < x) + 1;
        let v = tree.query(i) + 1;
        ans.push(v);
        tree.update(i, v);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        longest_obstacle_course_at_each_position(vec![1, 2, 3, 2])
    );
}

#[cfg(test)]
mod tests {
    use super::longest_obstacle_course_at_each_position;

    #[test]
    fn example_one() {
        assert_eq!(
            longest_obstacle_course_at_each_position(vec![1, 2, 3, 2]),
            vec![1, 2, 3, 3]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            longest_obstacle_course_at_each_position(vec![2, 2, 1]),
            vec![1, 2, 1]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            longest_obstacle_course_at_each_position(vec![3, 1, 5, 6, 4, 2]),
            vec![1, 1, 2, 3, 2, 2]
        );
    }
}
