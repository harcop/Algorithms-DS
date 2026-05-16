/// LeetCode #851 - Loud and Rich
fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
    let n = quiet.len();
    let mut g = vec![vec![]; n];
    let mut indeg = vec![0; n];
    for e in richer {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[b].push(a);
        indeg[a] += 1;
    }
    let mut ans: Vec<usize> = (0..n).collect();
    let mut q = std::collections::VecDeque::new();
    for i in 0..n {
        if indeg[i] == 0 {
            q.push_back(i);
        }
    }
    while let Some(u) = q.pop_front() {
        for &v in &g[u] {
            if quiet[ans[v]] > quiet[ans[u]] {
                ans[v] = ans[u];
            }
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    ans.into_iter().map(|x| x as i32).collect()
}

fn main() {
    println!(
        "{:?}",
        loud_and_rich(vec![vec![1, 0], vec![2, 1]], vec![1, 0, 2])
    );
}

#[cfg(test)]
mod tests {
    use super::loud_and_rich;

    #[test]
    fn example_one() {
        assert_eq!(
            loud_and_rich(vec![vec![1, 0], vec![2, 1]], vec![1, 0, 2]),
            vec![0, 1, 1]
        );
    }
}
