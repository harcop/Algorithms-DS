/// LeetCode #3367 - Maximize Sum of Weights after Edge Removals
fn maximize_sum_of_weights(edges: Vec<Vec<i32>>, k: i32) -> i64 {
    let n = edges.len() + 1;
    let mut g = vec![vec![]; n];
    for e in &edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        let w = e[2] as i64;
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let k = k as usize;
    fn dfs(u: usize, fa: i32, g: &[Vec<(usize, i64)>], k: usize) -> (i64, i64) {
        let mut s = 0i64;
        let mut t = Vec::new();
        for &(v, w) in &g[u] {
            if v as i32 == fa {
                continue;
            }
            let (a, b) = dfs(v, u as i32, g, k);
            s += a;
            let d = w + b - a;
            if d > 0 {
                t.push(d);
            }
        }
        t.sort_unstable_by(|a, b| b.cmp(a));
        let take_k: i64 = t.iter().take(k).sum();
        let take_km1: i64 = t.iter().take(k.saturating_sub(1)).sum();
        (s + take_k, s + take_km1)
    }
    let (x, y) = dfs(0, -1, &g, k);
    x.max(y)
}

fn main() {
    println!(
        "{}",
        maximize_sum_of_weights(vec![vec![0, 1, 4], vec![0, 2, 2], vec![2, 3, 12], vec![2, 4, 6]], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::maximize_sum_of_weights;

    #[test]
    fn example1() {
        assert_eq!(
            maximize_sum_of_weights(
                vec![vec![0, 1, 4], vec![0, 2, 2], vec![2, 3, 12], vec![2, 4, 6]],
                2
            ),
            22
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            maximize_sum_of_weights(
                vec![
                    vec![0, 1, 5],
                    vec![1, 2, 10],
                    vec![0, 3, 15],
                    vec![3, 4, 20],
                    vec![3, 5, 5],
                    vec![0, 6, 10]
                ],
                3
            ),
            65
        );
    }
}
