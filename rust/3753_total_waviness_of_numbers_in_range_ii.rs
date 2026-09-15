/// LeetCode #3753 - Total Waviness of Numbers in Range II
use std::collections::HashMap;

fn total_waviness(num1: i64, num2: i64) -> i64 {
    calc(num2) - calc(num1 - 1)
}

fn calc(x: i64) -> i64 {
    if x < 0 {
        return 0;
    }
    let mut digits = Vec::new();
    if x == 0 {
        digits.push(0i32);
    } else {
        let mut t = x;
        while t > 0 {
            digits.push((t % 10) as i32);
            t /= 10;
        }
        digits.reverse();
    }
    let mut memo: HashMap<(usize, i32, i32, i32, bool), (i64, i64)> = HashMap::new();
    dfs(0, 10, 10, 0, true, &digits, &mut memo).1
}

fn dfs(
    pos: usize,
    prev2: i32,
    prev1: i32,
    started: i32,
    limit: bool,
    digits: &[i32],
    memo: &mut HashMap<(usize, i32, i32, i32, bool), (i64, i64)>,
) -> (i64, i64) {
    if pos == digits.len() {
        return (started as i64, 0);
    }
    let key = (pos, prev2, prev1, started, limit);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let up = if limit { digits[pos] } else { 9 };
    let mut cnt = 0i64;
    let mut wav = 0i64;
    for d in 0..=up {
        let nlimit = limit && d == up;
        let (ns, np2, np1, add) = if started == 0 {
            if d == 0 {
                (0, 10, 10, 0)
            } else {
                (1, 10, d, 0)
            }
        } else {
            let add = if prev2 != 10
                && ((prev1 > prev2 && prev1 > d) || (prev1 < prev2 && prev1 < d))
            {
                1
            } else {
                0
            };
            (1, prev1, d, add)
        };
        let (c, w) = dfs(pos + 1, np2, np1, ns, nlimit, digits, memo);
        cnt += c;
        wav += w + c * add;
    }
    memo.insert(key, (cnt, wav));
    (cnt, wav)
}

fn main() {
    println!("{}", total_waviness(120, 130));
}

#[cfg(test)]
mod tests {
    use super::total_waviness;

    #[test]
    fn example1() {
        assert_eq!(total_waviness(120, 130), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(total_waviness(198, 202), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(total_waviness(4848, 4848), 2);
    }
}
