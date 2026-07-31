/// LeetCode #2827 - Number of Beautiful Integers in the Range
fn number_of_beautiful_integers(low: i32, high: i32, k: i32) -> i32 {
    count_up_to(high, k) - count_up_to(low - 1, k)
}

fn count_up_to(n: i32, k: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    let digits: Vec<i32> = n
        .to_string()
        .chars()
        .map(|c| (c as u8 - b'0') as i32)
        .collect();
    let len = digits.len();
    let max_diff = len as i32;
    let mut memo = vec![vec![vec![vec![-1i32; 4]; (max_diff * 2 + 1) as usize]; k as usize]; len + 1];

    fn dfs(
        pos: usize,
        sum_mod: i32,
        diff: i32,
        tight: bool,
        started: bool,
        digits: &[i32],
        k: i32,
        max_diff: i32,
        memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
    ) -> i32 {
        if pos == digits.len() {
            return if started && sum_mod == 0 && diff == 0 {
                1
            } else {
                0
            };
        }
        let ti = tight as usize;
        let si = started as usize;
        let di = (diff + max_diff) as usize;
        let cached = memo[pos][sum_mod as usize][di][ti * 2 + si];
        if cached >= 0 {
            return cached;
        }

        let limit = if tight { digits[pos] } else { 9 };
        let mut ans = 0;
        for d in 0..=limit {
            let new_tight = tight && d == limit;
            let new_started = started || d > 0;
            let new_sum = if new_started {
                (sum_mod + d) % k
            } else {
                sum_mod
            };
            let new_diff = if new_started {
                diff + if d % 2 == 0 { 1 } else { -1 }
            } else {
                diff
            };
            ans += dfs(
                pos + 1,
                new_sum,
                new_diff,
                new_tight,
                new_started,
                digits,
                k,
                max_diff,
                memo,
            );
        }
        memo[pos][sum_mod as usize][di][ti * 2 + si] = ans;
        ans
    }

    dfs(0, 0, 0, true, false, &digits, k, max_diff, &mut memo)
}

fn main() {
    println!("{}", number_of_beautiful_integers(10, 20, 3));
}

#[cfg(test)]
mod tests {
    use super::number_of_beautiful_integers;

    #[test]
    fn example_one() {
        assert_eq!(number_of_beautiful_integers(10, 20, 3), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_beautiful_integers(1, 10, 1), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(number_of_beautiful_integers(5, 5, 2), 0);
    }
}
