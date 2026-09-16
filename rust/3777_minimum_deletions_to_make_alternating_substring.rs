/// LeetCode #3777 - Minimum Deletions to Make Alternating Substring
struct Bit {
    n: usize,
    c: Vec<i32>,
}

impl Bit {
    fn new(n: usize) -> Self {
        Self {
            n,
            c: vec![0; n + 1],
        }
    }
    fn update(&mut self, mut x: usize, delta: i32) {
        while x <= self.n {
            self.c[x] += delta;
            x += x & x.wrapping_neg();
        }
    }
    fn query(&self, mut x: usize) -> i32 {
        let mut s = 0;
        while x > 0 {
            s += self.c[x];
            x -= x & x.wrapping_neg();
        }
        s
    }
}

fn min_deletions(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = s.len();
    let mut nums = vec![0i32; n];
    let b = s.as_bytes();
    let mut bit = Bit::new(n);
    for i in 1..n {
        nums[i] = if b[i] == b[i - 1] { 1 } else { 0 };
        if nums[i] == 1 {
            bit.update(i + 1, 1);
        }
    }
    let mut ans = Vec::new();
    for q in queries {
        if q[0] == 1 {
            let j = q[1] as usize;
            let delta = (nums[j] ^ 1) - nums[j];
            nums[j] ^= 1;
            bit.update(j + 1, delta);
            if j + 1 < n {
                let delta = (nums[j + 1] ^ 1) - nums[j + 1];
                nums[j + 1] ^= 1;
                bit.update(j + 2, delta);
            }
        } else {
            let l = q[1] as usize;
            let r = q[2] as usize;
            ans.push(bit.query(r + 1) - bit.query(l + 1));
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        min_deletions("ABA".into(), vec![vec![2, 1, 2], vec![1, 1], vec![2, 0, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::min_deletions;

    #[test]
    fn example1() {
        assert_eq!(
            min_deletions("ABA".into(), vec![vec![2, 1, 2], vec![1, 1], vec![2, 0, 2]]),
            vec![0, 2]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_deletions("ABB".into(), vec![vec![2, 0, 2], vec![1, 2], vec![2, 0, 2]]),
            vec![1, 0]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            min_deletions("BABA".into(), vec![vec![2, 0, 3], vec![1, 1], vec![2, 1, 3]]),
            vec![0, 1]
        );
    }
}
