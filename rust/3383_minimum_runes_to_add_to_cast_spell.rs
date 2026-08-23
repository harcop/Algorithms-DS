/// LeetCode #3383 - Minimum Runes to Add to Cast Spell
use std::collections::VecDeque;

fn min_runes_to_add(n: i32, crystals: Vec<i32>, flow_from: Vec<i32>, flow_to: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for (a, b) in flow_from.iter().zip(flow_to.iter()) {
        g[*a as usize].push(*b as usize);
    }
    let mut vis = vec![0i32; n];
    let mut q = VecDeque::new();
    for &x in &crystals {
        vis[x as usize] = 1;
        q.push_back(x as usize);
    }
    bfs(&g, &mut vis, &mut q);
    let mut seq = Vec::new();
    for i in 0..n {
        if vis[i] == 0 {
            dfs(&g, &mut vis, &mut seq, i);
        }
    }
    seq.reverse();
    let mut ans = 0;
    for &i in &seq {
        if vis[i] == 2 {
            vis[i] = 1;
            q.clear();
            q.push_back(i);
            bfs(&g, &mut vis, &mut q);
            ans += 1;
        }
    }
    ans
}

fn bfs(g: &[Vec<usize>], vis: &mut [i32], q: &mut VecDeque<usize>) {
    while let Some(a) = q.pop_front() {
        for &b in &g[a] {
            if vis[b] == 1 {
                continue;
            }
            vis[b] = 1;
            q.push_back(b);
        }
    }
}

fn dfs(g: &[Vec<usize>], vis: &mut [i32], seq: &mut Vec<usize>, a: usize) {
    vis[a] = 2;
    for &b in &g[a] {
        if vis[b] > 0 {
            continue;
        }
        dfs(g, vis, seq, b);
    }
    seq.push(a);
}

fn main() {
    println!(
        "{}",
        min_runes_to_add(6, vec![0], vec![0, 1, 2, 3], vec![1, 2, 3, 0])
    );
}

#[cfg(test)]
mod tests {
    use super::min_runes_to_add;

    #[test]
    fn example1() {
        assert_eq!(
            min_runes_to_add(6, vec![0], vec![0, 1, 2, 3], vec![1, 2, 3, 0]),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_runes_to_add(7, vec![3, 5], vec![0, 1, 2, 3, 5], vec![1, 2, 0, 4, 6]),
            1
        );
    }
}
