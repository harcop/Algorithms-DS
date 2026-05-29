/// LeetCode #1505 - Minimum Possible Integer After At Most K Adjacent Swaps On Digits
struct Fenwick {
    n: usize,
    bit: Vec<i32>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick { n, bit: vec![0; n + 1] }
    }
    fn add(&mut self, mut i: usize, delta: i32) {
        i += 1;
        while i <= self.n {
            self.bit[i] += delta;
            i += i & i.wrapping_neg();
        }
    }
    fn sum(&self, mut i: usize) -> i32 {
        i += 1;
        let mut s = 0;
        while i > 0 {
            s += self.bit[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
}

fn min_integer(num: String, k: i32) -> String {
    let digits: Vec<u8> = num.bytes().collect();
    let n = digits.len();
    let mut pos: Vec<Vec<usize>> = vec![vec![]; 10];
    for (i, &d) in digits.iter().enumerate() {
        pos[(d - b'0') as usize].push(i);
    }
    let mut fw = Fenwick::new(n);
    let mut used = vec![false; n];
    let mut ans = Vec::with_capacity(n);
    let mut rem = k as i64;
    for _ in 0..n {
        for d in 0..10u8 {
            let idxs = &pos[d as usize];
            let mut pick = None;
            for &idx in idxs {
                if used[idx] {
                    continue;
                }
                let shifted = idx as i64 - fw.sum(idx) as i64;
                if shifted <= rem {
                    pick = Some((idx, shifted));
                    break;
                }
            }
            if let Some((idx, shifted)) = pick {
                rem -= shifted;
                used[idx] = true;
                fw.add(idx, 1);
                ans.push(d + b'0');
                break;
            }
        }
    }
    String::from_utf8(ans).unwrap()
}

fn main() {
    println!("{}", min_integer("4321".into(), 4));
}

#[cfg(test)]
mod tests {
    use super::min_integer;

    #[test]
    fn example_one() {
        assert_eq!(min_integer("4321".into(), 4), "1342");
    }

    #[test]
    fn example_two() {
        assert_eq!(min_integer("100".into(), 1), "010");
    }
}
