/// LeetCode #1900 - The Earliest and Latest Rounds Where Players Compete
use std::collections::HashMap;

const INF: i32 = i32::MAX / 2;

fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
    let mut memo = HashMap::new();
    let (a, b) = dfs(
        (first_player - 1) as usize,
        (second_player - 1) as usize,
        n as usize,
        &mut memo,
    );
    vec![a, b]
}

fn dfs(
    l: usize,
    r: usize,
    n: usize,
    memo: &mut HashMap<(usize, usize, usize), (i32, i32)>,
) -> (i32, i32) {
    if let Some(&v) = memo.get(&(l, r, n)) {
        return v;
    }
    if l + r == n - 1 {
        return (1, 1);
    }
    let mut res = (INF, i32::MIN / 2);
    let m = n >> 1;
    for i in 0..(1 << m) {
        let mut win = vec![false; n];
        for j in 0..m {
            if i >> j & 1 == 1 {
                win[j] = true;
            } else {
                win[n - 1 - j] = true;
            }
        }
        if n & 1 == 1 {
            win[m] = true;
        }
        win[n - 1 - l] = false;
        win[n - 1 - r] = false;
        win[l] = true;
        win[r] = true;
        let mut a = 0usize;
        let mut b = 0usize;
        let mut c = 0usize;
        for j in 0..n {
            if j == l {
                a = c;
            }
            if j == r {
                b = c;
            }
            if win[j] {
                c += 1;
            }
        }
        let (x, y) = dfs(a, b, c, memo);
        res.0 = res.0.min(x + 1);
        res.1 = res.1.max(y + 1);
    }
    memo.insert((l, r, n), res);
    res
}

fn main() {
    println!("{:?}", earliest_and_latest(11, 2, 4));
}

#[cfg(test)]
mod tests {
    use super::earliest_and_latest;

    #[test]
    fn example_one() {
        assert_eq!(earliest_and_latest(11, 2, 4), vec![3, 4]);
    }

    #[test]
    fn example_two() {
        assert_eq!(earliest_and_latest(5, 1, 5), vec![1, 1]);
    }
}
