/// LeetCode #2247 - Maximum Cost of Trip With K Highways
use std::collections::HashMap;

fn maximum_cost(n: i32, highways: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    if k + 1 > n {
        return -1;
    }

    let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
    for h in highways {
        let u = h[0] as usize;
        let v = h[1] as usize;
        let w = h[2];
        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    let mut memo: HashMap<(usize, u32), i32> = HashMap::new();
    let mut ans = -1;
    for i in 0..n {
        ans = ans.max(dp(i, 1u32 << i, k, &graph, &mut memo));
    }
    ans
}

fn dp(
    u: usize,
    mask: u32,
    k: usize,
    graph: &[Vec<(usize, i32)>],
    memo: &mut HashMap<(usize, u32), i32>,
) -> i32 {
    if mask.count_ones() as usize == k + 1 {
        return 0;
    }
    if let Some(&cached) = memo.get(&(u, mask)) {
        return cached;
    }

    let mut res = -1;
    for &(v, w) in &graph[u] {
        if mask & (1 << v) != 0 {
            continue;
        }
        let next = dp(v, mask | (1 << v), k, graph, memo);
        if next != -1 {
            res = res.max(w + next);
        }
    }

    memo.insert((u, mask), res);
    res
}

fn main() {
    println!(
        "{}",
        maximum_cost(
            4,
            vec![vec![0, 1, 12], vec![0, 2, 10], vec![0, 3, 40]],
            2
        )
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_cost;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_cost(
                4,
                vec![vec![0, 1, 12], vec![0, 2, 10], vec![0, 3, 40]],
                2
            ),
            52
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_cost(4, vec![vec![0, 1, 10], vec![1, 2, 20], vec![2, 3, 30]], 2),
            50
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            maximum_cost(
                4,
                vec![vec![0, 1, 10], vec![1, 2, 20], vec![2, 3, 30], vec![0, 3, 40]],
                2
            ),
            70
        );
    }
}
