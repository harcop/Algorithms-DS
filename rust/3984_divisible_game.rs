/// LeetCode #3984 - Divisible Game
use std::collections::HashMap;

fn divisible_game(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = nums.len();
    let mut pref = vec![0i64; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i] + nums[i] as i64;
    }
    let mut groups: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        if x <= 1 {
            continue;
        }
        let mut d = 1i32;
        while d * d <= x {
            if x % d == 0 {
                if d > 1 {
                    groups.entry(d).or_default().push(i);
                }
                let other = x / d;
                if other > 1 && other != d {
                    groups.entry(other).or_default().push(i);
                }
            }
            d += 1;
        }
    }
    let mut best_diff = -(*nums.iter().min().unwrap()) as i64;
    let mut best_k = 2i64;
    for (k, pos) in groups {
        let mut best_l: Option<i64> = None;
        let mut mult = 0i64;
        let mut ans: Option<i64> = None;
        for &p in &pos {
            let term_l = pref[p] - 2 * mult;
            best_l = Some(best_l.map_or(term_l, |v| v.max(term_l)));
            mult += nums[p] as i64;
            let term_r = 2 * mult - pref[p + 1];
            let val = term_r + best_l.unwrap();
            ans = Some(ans.map_or(val, |v| v.max(val)));
        }
        let ans = ans.unwrap();
        let k = k as i64;
        if ans > best_diff || (ans == best_diff && k < best_k) {
            best_diff = ans;
            best_k = k;
        }
    }
    (best_diff * best_k).rem_euclid(MOD) as i32
}

fn main() {
    println!("{}", divisible_game(vec![1, 4, 6, 8]));
}

#[cfg(test)]
mod tests {
    use super::divisible_game;

    #[test]
    fn example1() {
        assert_eq!(divisible_game(vec![1, 4, 6, 8]), 36);
    }

    #[test]
    fn example2() {
        assert_eq!(divisible_game(vec![2, 1, 2]), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(divisible_game(vec![1]), 1_000_000_005);
    }

    #[test]
    fn single_composite() {
        assert_eq!(divisible_game(vec![6]), 12);
    }
}
