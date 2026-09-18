/// LeetCode #3791 - Number of Balanced Integers in a Range
fn count_balanced(low: i64, high: i64) -> i64 {
    if high < 11 {
        return 0;
    }
    let low = low.max(11);
    const BASE: i32 = 90;
    fn dfs(pos: usize, diff: i32, lim: bool, num: &[u8], f: &mut [Vec<i64>]) -> i64 {
        if pos >= num.len() {
            return if diff == 0 { 1 } else { 0 };
        }
        if !lim && f[pos][(diff + BASE) as usize] != -1 {
            return f[pos][(diff + BASE) as usize];
        }
        let up = if lim { (num[pos] - b'0') as i32 } else { 9 };
        let mut res = 0i64;
        for i in 0..=up {
            let nd = diff + i * if pos % 2 == 0 { 1 } else { -1 };
            res += dfs(pos + 1, nd, lim && i == up, num, f);
        }
        if !lim {
            f[pos][(diff + BASE) as usize] = res;
        }
        res
    }
    let num_a = (low - 1).to_string().into_bytes();
    let mut f_a = vec![vec![-1i64; 181]; num_a.len()];
    let a = dfs(0, 0, true, &num_a, &mut f_a);
    let num_b = high.to_string().into_bytes();
    let mut f_b = vec![vec![-1i64; 181]; num_b.len()];
    let b = dfs(0, 0, true, &num_b, &mut f_b);
    b - a
}

fn main() {
    println!("{}", count_balanced(1, 100));
}

#[cfg(test)]
mod tests {
    use super::count_balanced;

    #[test]
    fn example1() {
        assert_eq!(count_balanced(1, 100), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(count_balanced(120, 129), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(count_balanced(1234, 1234), 0);
    }
}
