/// LeetCode #1718 - Construct the Lexicographically Largest Valid Sequence
fn construct_distanced_sequence(n: i32) -> Vec<i32> {
    let n = n as usize;
    let len = 2 * n - 1;
    let mut ans = vec![0i32; len];
    let mut used = vec![false; n + 1];
    fn dfs(pos: usize, n: usize, ans: &mut [i32], used: &mut [bool]) -> bool {
        if pos == ans.len() {
            return true;
        }
        if ans[pos] != 0 {
            return dfs(pos + 1, n, ans, used);
        }
        for v in (1..=n).rev() {
            if used[v] {
                continue;
            }
            if v == 1 {
                ans[pos] = 1;
                used[1] = true;
                if dfs(pos + 1, n, ans, used) {
                    return true;
                }
                used[1] = false;
                ans[pos] = 0;
            } else if pos + v < ans.len() && ans[pos + v] == 0 {
                ans[pos] = v as i32;
                ans[pos + v] = v as i32;
                used[v] = true;
                if dfs(pos + 1, n, ans, used) {
                    return true;
                }
                used[v] = false;
                ans[pos] = 0;
                ans[pos + v] = 0;
            }
        }
        false
    }
    dfs(0, n, &mut ans, &mut used);
    ans
}
fn main() {
    println!("{:?}", construct_distanced_sequence(3));
}
#[cfg(test)]
mod tests {
    use super::construct_distanced_sequence;
    #[test]
    fn example_one() {
        assert_eq!(construct_distanced_sequence(3), vec![3, 1, 2, 3, 2]);
    }
    #[test]
    fn example_two() {
        assert_eq!(construct_distanced_sequence(5), vec![5, 3, 1, 4, 3, 5, 2, 4, 2]);
    }
}
