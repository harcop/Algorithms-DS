/// LeetCode #3972 - Valid Subarrays With Matching Sum Digits II
fn count_valid_subarrays(nums: Vec<i32>, x: i32) -> i64 {
    let mut pref = vec![0i64];
    for &v in &nums {
        pref.push(pref.last().unwrap() + v as i64);
    }
    let mut vals = pref.clone();
    vals.sort_unstable();
    vals.dedup();

    struct Fenwick {
        bit: Vec<i64>,
    }
    impl Fenwick {
        fn new(n: usize) -> Self {
            Self { bit: vec![0; n + 1] }
        }
        fn add(&mut self, mut i: usize) {
            i += 1;
            while i < self.bit.len() {
                self.bit[i] += 1;
                i += i & i.wrapping_neg();
            }
        }
        fn prefix(&self, i: i32) -> i64 {
            if i < 0 {
                return 0;
            }
            let mut i = i as usize + 1;
            let mut s = 0i64;
            while i > 0 {
                s += self.bit[i];
                i -= i & i.wrapping_neg();
            }
            s
        }
    }

    let mut fen: Vec<Fenwick> = (0..10).map(|_| Fenwick::new(vals.len())).collect();
    let rank = |v: i64| vals.binary_search(&v).unwrap();
    fen[0].add(rank(0));
    let x = x as i64;
    let mut ans = 0i64;
    for r in 1..pref.len() {
        let p = pref[r];
        let need = ((p % 10) - x + 10) % 10;
        let mut pow = 1i64;
        loop {
            let left = x * pow;
            if left > p {
                break;
            }
            let right = left + pow;
            let mut lo = p - right + 1;
            let hi = p - left;
            if lo < 0 {
                lo = 0;
            }
            if lo <= hi {
                let l = vals.partition_point(|&v| v < lo);
                let h = vals.partition_point(|&v| v <= hi);
                if l < h {
                    ans += fen[need as usize].prefix(h as i32 - 1)
                        - fen[need as usize].prefix(l as i32 - 1);
                }
            }
            if pow > p / 10 {
                break;
            }
            pow *= 10;
        }
        fen[(p % 10) as usize].add(rank(p));
    }
    ans
}

fn main() {
    println!("{}", count_valid_subarrays(vec![1, 100, 1], 1));
}

#[cfg(test)]
mod tests {
    use super::count_valid_subarrays;

    #[test]
    fn example1() {
        assert_eq!(count_valid_subarrays(vec![1, 100, 1], 1), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(count_valid_subarrays(vec![1], 2), 0);
    }
}
