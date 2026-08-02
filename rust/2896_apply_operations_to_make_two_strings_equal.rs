/// LeetCode #2896 - Apply Operations to Make Two Strings Equal
fn min_operations(s1: String, s2: String, x: i32) -> i32 {
    let idx: Vec<i32> = s1
        .bytes()
        .zip(s2.bytes())
        .enumerate()
        .filter_map(|(i, (a, b))| if a != b { Some(i as i32) } else { None })
        .collect();
    let m = idx.len();
    if m % 2 == 1 {
        return -1;
    }
    if m == 0 {
        return 0;
    }

    let mut memo = vec![vec![-1; m]; m];
    dfs(0, (m - 1) as i32, &idx, x, &mut memo)
}

fn dfs(i: i32, j: i32, idx: &[i32], x: i32, memo: &mut [Vec<i32>]) -> i32 {
    if i > j {
        return 0;
    }
    let (ui, uj) = (i as usize, j as usize);
    if memo[ui][uj] != -1 {
        return memo[ui][uj];
    }
    let a = dfs(i + 1, j - 1, idx, x, memo) + x;
    let b = dfs(i + 2, j, idx, x, memo) + idx[ui + 1] - idx[ui];
    let c = dfs(i, j - 2, idx, x, memo) + idx[uj] - idx[uj - 1];
    memo[ui][uj] = a.min(b).min(c);
    memo[ui][uj]
}

fn main() {
    println!(
        "{}",
        min_operations("1100011000".into(), "0101001010".into(), 2)
    );
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example_one() {
        assert_eq!(
            min_operations("1100011000".into(), "0101001010".into(), 2),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(min_operations("10110".into(), "00011".into(), 4), -1);
    }
}
