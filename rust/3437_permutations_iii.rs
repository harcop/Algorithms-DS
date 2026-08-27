/// LeetCode #3437 - Permutations III
fn permute(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut ans = Vec::new();
    let mut t = Vec::new();
    let mut vis = vec![false; n + 1];
    fn dfs(i: usize, n: usize, t: &mut Vec<i32>, vis: &mut [bool], ans: &mut Vec<Vec<i32>>) {
        if i >= n {
            ans.push(t.clone());
            return;
        }
        for j in 1..=n as i32 {
            if !vis[j as usize] && (i == 0 || t[i - 1] % 2 != j % 2) {
                t.push(j);
                vis[j as usize] = true;
                dfs(i + 1, n, t, vis, ans);
                vis[j as usize] = false;
                t.pop();
            }
        }
    }
    dfs(0, n, &mut t, &mut vis, &mut ans);
    ans
}

fn main() {
    println!("{:?}", permute(4));
}

#[cfg(test)]
mod tests {
    use super::permute;

    #[test]
    fn example1() {
        assert_eq!(
            permute(4),
            vec![
                vec![1, 2, 3, 4],
                vec![1, 4, 3, 2],
                vec![2, 1, 4, 3],
                vec![2, 3, 4, 1],
                vec![3, 2, 1, 4],
                vec![3, 4, 1, 2],
                vec![4, 1, 2, 3],
                vec![4, 3, 2, 1],
            ]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(permute(2), vec![vec![1, 2], vec![2, 1]]);
    }

    #[test]
    fn example3() {
        assert_eq!(permute(3), vec![vec![1, 2, 3], vec![3, 2, 1]]);
    }
}
