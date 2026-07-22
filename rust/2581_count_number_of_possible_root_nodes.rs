/// LeetCode #2581 - Count Number of Possible Root Nodes
use std::collections::HashMap;

fn root_count(edges: Vec<Vec<i32>>, guesses: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = edges.len() + 1;
    let mut g = vec![Vec::new(); n];
    for e in &edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push(b);
        g[b].push(a);
    }

    let mut gs: HashMap<(usize, usize), i32> = HashMap::new();
    for e in &guesses {
        *gs.entry((e[0] as usize, e[1] as usize)).or_insert(0) += 1;
    }

    let mut cnt = 0;
    fn dfs1(
        i: usize,
        fa: i32,
        g: &[Vec<usize>],
        gs: &HashMap<(usize, usize), i32>,
        cnt: &mut i32,
    ) {
        for &j in &g[i] {
            if j as i32 != fa {
                *cnt += gs.get(&(i, j)).copied().unwrap_or(0);
                dfs1(j, i as i32, g, gs, cnt);
            }
        }
    }
    dfs1(0, -1, &g, &gs, &mut cnt);

    let mut ans = 0;
    fn dfs2(
        i: usize,
        fa: i32,
        g: &[Vec<usize>],
        gs: &HashMap<(usize, usize), i32>,
        cnt: &mut i32,
        k: i32,
        ans: &mut i32,
    ) {
        if *cnt >= k {
            *ans += 1;
        }
        for &j in &g[i] {
            if j as i32 != fa {
                let a = gs.get(&(i, j)).copied().unwrap_or(0);
                let b = gs.get(&(j, i)).copied().unwrap_or(0);
                *cnt -= a;
                *cnt += b;
                dfs2(j, i as i32, g, gs, cnt, k, ans);
                *cnt -= b;
                *cnt += a;
            }
        }
    }
    dfs2(0, -1, &g, &gs, &mut cnt, k, &mut ans);
    ans
}

fn main() {
    println!(
        "{}",
        root_count(
            vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![4, 2]],
            vec![vec![1, 3], vec![0, 1], vec![1, 0], vec![2, 4]],
            3
        )
    );
}

#[cfg(test)]
mod tests {
    use super::root_count;

    #[test]
    fn example_one() {
        assert_eq!(
            root_count(
                vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![4, 2]],
                vec![vec![1, 3], vec![0, 1], vec![1, 0], vec![2, 4]],
                3
            ),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            root_count(
                vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]],
                vec![vec![1, 0], vec![3, 4], vec![2, 1], vec![3, 2]],
                1
            ),
            5
        );
    }
}
