/// LeetCode #1976 - Number of Ways to Arrive at Destination
const MOD: i64 = 1_000_000_007;
const INF: i64 = i64::MAX / 4;

fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut g = vec![vec![INF; n]; n];
    for i in 0..n {
        g[i][i] = 0;
    }
    for r in roads {
        let u = r[0] as usize;
        let v = r[1] as usize;
        let t = r[2] as i64;
        g[u][v] = t;
        g[v][u] = t;
    }

    let mut dist = vec![INF; n];
    dist[0] = 0;
    let mut ways = vec![0i64; n];
    ways[0] = 1;

    let mut vis = vec![false; n];
    for _ in 0..n {
        let mut t = None;
        for j in 0..n {
            if !vis[j] && (t.is_none() || dist[j] < dist[t.unwrap()]) {
                t = Some(j);
            }
        }
        let Some(t) = t else {
            break;
        };
        vis[t] = true;
        for j in 0..n {
            if j == t {
                continue;
            }
            let ne = dist[t] + g[t][j];
            if dist[j] > ne {
                dist[j] = ne;
                ways[j] = ways[t];
            } else if dist[j] == ne {
                ways[j] = (ways[j] + ways[t]) % MOD;
            }
        }
    }
    ways[n - 1] as i32
}

fn main() {
    println!(
        "{}",
        count_paths(
            7,
            vec![
                vec![0, 6, 7],
                vec![0, 1, 2],
                vec![1, 2, 3],
                vec![1, 3, 3],
                vec![6, 3, 3],
                vec![3, 5, 1],
                vec![6, 5, 1],
                vec![2, 5, 1],
                vec![0, 4, 5],
                vec![4, 6, 2],
            ],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::count_paths;

    #[test]
    fn example_one() {
        assert_eq!(
            count_paths(
                7,
                vec![
                    vec![0, 6, 7],
                    vec![0, 1, 2],
                    vec![1, 2, 3],
                    vec![1, 3, 3],
                    vec![6, 3, 3],
                    vec![3, 5, 1],
                    vec![6, 5, 1],
                    vec![2, 5, 1],
                    vec![0, 4, 5],
                    vec![4, 6, 2],
                ],
            ),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(count_paths(2, vec![vec![1, 0, 10]]), 1);
    }
}
