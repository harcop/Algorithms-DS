/// LeetCode #3272 - Find the Count of Good Integers
use std::collections::HashSet;

fn count_good_integers(n: i32, k: i32) -> i64 {
    let n = n as usize;
    let k = k as i64;
    let mut fac = vec![1i64; n + 1];
    for i in 1..=n {
        fac[i] = fac[i - 1] * i as i64;
    }
    let mut ans = 0i64;
    let mut vis = HashSet::new();
    let base = 10i64.pow(((n - 1) / 2) as u32);
    for i in base..base * 10 {
        let s = i.to_string();
        let rev: String = s.chars().rev().collect();
        let full_s = if n % 2 == 0 {
            format!("{}{}", s, rev)
        } else {
            format!("{}{}", s, &rev[1..])
        };
        let num: i64 = full_s.parse().unwrap();
        if num % k != 0 {
            continue;
        }
        let mut arr: Vec<char> = full_s.chars().collect();
        arr.sort_unstable();
        let t: String = arr.iter().collect();
        if vis.contains(&t) {
            continue;
        }
        vis.insert(t);
        let mut cnt = vec![0; 10];
        for c in &arr {
            cnt[*c as usize - '0' as usize] += 1;
        }
        let mut res = (n - cnt[0]) as i64 * fac[n - 1];
        for &x in &cnt {
            if x > 0 {
                res /= fac[x];
            }
        }
        ans += res;
    }
    ans
}

fn main() {
    println!("{}", count_good_integers(3, 5));
}

#[cfg(test)]
mod tests {
    use super::count_good_integers;

    #[test]
    fn example1() {
        assert_eq!(count_good_integers(3, 5), 27);
    }

    #[test]
    fn example2() {
        assert_eq!(count_good_integers(1, 4), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(count_good_integers(5, 6), 2468);
    }
}
