/// LeetCode #277 - Find the Celebrity (graph knows API mocked for tests)
pub struct Solution {
    celeb: usize,
    n: usize,
}

impl Solution {
    fn new(n: usize, celeb: usize) -> Self {
        Solution { celeb, n }
    }

    fn knows(&self, a: usize, b: usize) -> bool {
        b == self.celeb && a != self.celeb
    }

    fn find_celebrity(&self) -> i32 {
        let mut cand = 0usize;
        for j in 1..self.n {
            if self.knows(cand, j) {
                cand = j;
            }
        }
        for j in 0..self.n {
            if j != cand && (self.knows(cand, j) || !self.knows(j, cand)) {
                return -1;
            }
        }
        cand as i32
    }
}

fn main() {
    let s = Solution::new(3, 1);
    println!("{}", s.find_celebrity());
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_one() {
        let s = Solution::new(3, 1);
        assert_eq!(s.find_celebrity(), 1);
    }
}
