/// LeetCode #3490 - Count Beautiful Numbers
use std::collections::HashMap;

fn count(x: i32) -> i32 {
    if x <= 0 {
        return 0;
    }
    let digits: Vec<i32> = x
        .to_string()
        .bytes()
        .map(|c| (c - b'0') as i32)
        .collect();
    let mut dp: [HashMap<(i64, i32), i32>; 2] = [HashMap::new(), HashMap::new()];
    dp[1].insert((1, 0), 1);
    for &c in &digits {
        let mut new_dp: [HashMap<(i64, i32), i32>; 2] = [HashMap::new(), HashMap::new()];
        for b in 0..2 {
            for (&(mul, total), &cnt) in &dp[b] {
                let lim = if b == 1 { c } else { 9 };
                for d in 0..=lim {
                    let new_mul = mul * if total == 0 && d == 0 { 1 } else { d as i64 };
                    let new_total = total + d;
                    *new_dp[(b == 1 && d == c) as usize]
                        .entry((new_mul, new_total))
                        .or_insert(0) += cnt;
                }
            }
        }
        dp = new_dp;
    }
    let mut result = 0;
    for b in 0..2 {
        for (&(mul, total), &cnt) in &dp[b] {
            if total > 0 && mul % total as i64 == 0 {
                result += cnt;
            }
        }
    }
    result
}

fn beautiful_numbers(l: i32, r: i32) -> i32 {
    count(r) - count(l - 1)
}

fn main() {
    println!("{}", beautiful_numbers(10, 20));
}

#[cfg(test)]
mod tests {
    use super::beautiful_numbers;

    #[test]
    fn example1() {
        assert_eq!(beautiful_numbers(10, 20), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(beautiful_numbers(1, 15), 10);
    }
}
