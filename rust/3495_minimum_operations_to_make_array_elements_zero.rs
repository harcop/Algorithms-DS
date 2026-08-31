/// LeetCode #3495 - Minimum Operations to Make Array Elements Zero
fn f(x: i64) -> i64 {
    let mut res = 0i64;
    let mut p = 1i64;
    let mut i = 1i64;
    while p <= x {
        let cnt = (p * 4 - 1).min(x) - p + 1;
        res += cnt * i;
        i += 1;
        p *= 4;
    }
    res
}

fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
    let mut ans = 0i64;
    for q in queries {
        let l = q[0] as i64;
        let r = q[1] as i64;
        let s = f(r) - f(l - 1);
        let mx = f(r) - f(r - 1);
        ans += ((s + 1) / 2).max(mx);
    }
    ans
}

fn main() {
    println!("{}", min_operations(vec![vec![1, 2], vec![2, 4]]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![vec![1, 2], vec![2, 4]]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![vec![2, 6]]), 4);
    }
}
