/// LeetCode #3528 - Unit Conversion I
fn base_unit_conversions(conversions: Vec<Vec<i32>>) -> Vec<i32> {
    const MOD: i64 = 1_000_000_007;
    let n = conversions.len() + 1;
    let mut g = vec![Vec::new(); n];
    for c in &conversions {
        g[c[0] as usize].push((c[1] as usize, c[2] as i64));
    }
    let mut ans = vec![0i32; n];
    ans[0] = 1;
    let mut stack = vec![0usize];
    while let Some(s) = stack.pop() {
        for &(t, w) in &g[s] {
            ans[t] = ((ans[s] as i64) * w % MOD) as i32;
            stack.push(t);
        }
    }
    ans
}

fn main() {
    println!("{:?}", base_unit_conversions(vec![vec![0, 1, 2], vec![1, 2, 3]]));
}

#[cfg(test)]
mod tests {
    use super::base_unit_conversions;

    #[test]
    fn example1() {
        assert_eq!(
            base_unit_conversions(vec![vec![0, 1, 2], vec![1, 2, 3]]),
            vec![1, 2, 6]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            base_unit_conversions(vec![
                vec![0, 1, 2],
                vec![0, 2, 3],
                vec![1, 3, 4],
                vec![1, 4, 5],
                vec![2, 5, 2],
                vec![4, 6, 3],
                vec![5, 7, 4],
            ]),
            vec![1, 2, 3, 8, 10, 6, 30, 24]
        );
    }
}
