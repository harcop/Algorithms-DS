/// LeetCode #1340 - Jump Game V

fn can_reach(arr: Vec<i32>, start: i32) -> bool {
    let n = arr.len();
    let start = start as usize;
    let mut memo = vec![None; n];
    fn dfs(i: usize, arr: &[i32], n: usize, memo: &mut [Option<bool>]) -> bool {
        if i == n - 1 {
            return true;
        }
        if let Some(v) = memo[i] {
            return v;
        }
        let reach = arr[i] as usize;
        let mut ok = false;
        let lo = i.saturating_sub(reach);
        for j in lo..i {
            if arr[j] < arr[i] && dfs(j, arr, n, memo) {
                ok = true;
                break;
            }
        }
        if !ok {
            let hi = (i + reach).min(n - 1);
            for j in i + 1..=hi {
                if arr[j] < arr[i] && dfs(j, arr, n, memo) {
                    ok = true;
                    break;
                }
            }
        }
        memo[i] = Some(ok);
        ok
    }
    dfs(start, &arr, n, &mut memo)
}

fn main() {
    println!("{}", can_reach(vec![3, 2, 1, 4, 5], 2));
}

#[cfg(test)]
mod tests {
    use super::can_reach;

    #[test]
    fn example_one() {
        assert!(can_reach(vec![4, 2, 3, 0, 3, 1, 2], 6));
    }

    #[test]
    fn example_two() {
        assert!(!can_reach(vec![3, 2, 1, 4, 5], 3));
    }
}
