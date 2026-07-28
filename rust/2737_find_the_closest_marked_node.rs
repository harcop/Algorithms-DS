/// LeetCode #2737 - Find the Closest Marked Node
fn minimum_distance(n: i32, edges: Vec<Vec<i32>>, s: i32, marked: Vec<i32>) -> i32 {
    let n = n as usize;
    let inf = 1 << 29;
    let mut g = vec![vec![inf; n]; n];
    for e in &edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        let w = e[2] as i32;
        g[u][v] = g[u][v].min(w);
    }
    let mut dist = vec![inf; n];
    let mut vis = vec![false; n];
    dist[s as usize] = 0;
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
            dist[j] = dist[j].min(dist[t] + g[t][j]);
        }
    }
    let mut ans = inf;
    for &i in &marked {
        ans = ans.min(dist[i as usize]);
    }
    if ans >= inf {
        -1
    } else {
        ans
    }
}

fn main() {
    println!(
        "{}",
        minimum_distance(
            4,
            vec![vec![0, 1, 1], vec![1, 2, 3], vec![2, 3, 2], vec![0, 3, 4]],
            0,
            vec![2, 3]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_distance;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_distance(
                4,
                vec![vec![0, 1, 1], vec![1, 2, 3], vec![2, 3, 2], vec![0, 3, 4]],
                0,
                vec![2, 3]
            ),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_distance(
                5,
                vec![
                    vec![0, 1, 2],
                    vec![0, 2, 4],
                    vec![1, 3, 1],
                    vec![2, 3, 3],
                    vec![3, 4, 2]
                ],
                1,
                vec![0, 4]
            ),
            3
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            minimum_distance(4, vec![vec![0, 1, 1], vec![1, 2, 3], vec![2, 3, 2]], 3, vec![0, 1]),
            -1
        );
    }
}
