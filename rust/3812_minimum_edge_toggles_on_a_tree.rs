/// LeetCode #3812 - Minimum Edge Toggles on a Tree
fn minimum_flips(n: i32, edges: Vec<Vec<i32>>, start: String, target: String) -> Vec<i32> {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for (i, e) in edges.iter().enumerate() {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push((b, i as i32));
        g[b].push((a, i as i32));
    }
    let start = start.into_bytes();
    let target = target.into_bytes();
    let mut ans = Vec::new();
    fn dfs(
        a: usize,
        fa: i32,
        g: &[Vec<(usize, i32)>],
        start: &[u8],
        target: &[u8],
        ans: &mut Vec<i32>,
    ) -> bool {
        let mut rev = start[a] != target[a];
        for &(b, i) in &g[a] {
            if b as i32 != fa && dfs(b, a as i32, g, start, target, ans) {
                ans.push(i);
                rev = !rev;
            }
        }
        rev
    }
    if dfs(0, -1, &g, &start, &target, &mut ans) {
        return vec![-1];
    }
    ans.sort_unstable();
    ans
}

fn main() {
    println!(
        "{:?}",
        minimum_flips(3, vec![vec![0, 1], vec![1, 2]], "010".into(), "100".into())
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_flips;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_flips(3, vec![vec![0, 1], vec![1, 2]], "010".into(), "100".into()),
            vec![0]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            minimum_flips(
                7,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![2, 3],
                    vec![3, 4],
                    vec![3, 5],
                    vec![1, 6]
                ],
                "0011000".into(),
                "0010001".into()
            ),
            vec![1, 2, 5]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            minimum_flips(2, vec![vec![0, 1]], "00".into(), "01".into()),
            vec![-1]
        );
    }
}
