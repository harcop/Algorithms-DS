/// LeetCode #483 - Smallest Good Base
use std::cmp::Ordering;

fn smallest_good_base(n: String) -> String {
    let n: u128 = n.parse().unwrap();
    let max_len = (n as f64).log2() as u32 + 1;
    for m in (3..=max_len).rev() {
        let mut lo = 2u128;
        let mut hi = n - 1;
        while lo <= hi {
            let k = lo + (hi - lo) / 2;
            match cmp_geom(k, m, n) {
                Ordering::Equal => return k.to_string(),
                Ordering::Less => lo = k + 1,
                Ordering::Greater => {
                    if k == 0 {
                        break;
                    }
                    hi = k - 1;
                }
            }
        }
    }
    (n - 1).to_string()
}

fn cmp_geom(k: u128, m: u32, n: u128) -> Ordering {
    let mut sum = 1u128;
    let mut p = 1u128;
    for _ in 1..m {
        match p.checked_mul(k) {
            Some(np) => p = np,
            None => return Ordering::Greater,
        }
        match sum.checked_add(p) {
            Some(ns) => sum = ns,
            None => return Ordering::Greater,
        }
        if sum > n {
            return Ordering::Greater;
        }
    }
    sum.cmp(&n)
}

fn main() {
    println!("{}", smallest_good_base("13".into()));
}

#[cfg(test)]
mod tests {
    use super::smallest_good_base;

    #[test]
    fn example_one() {
        assert_eq!(smallest_good_base("13".into()), "3");
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_good_base("4681".into()), "8");
    }

    #[test]
    fn example_three() {
        assert_eq!(
            smallest_good_base("1000000000000000000".into()),
            "999999999999999999"
        );
    }
}
