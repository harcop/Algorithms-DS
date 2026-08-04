/// LeetCode #2959 - Number of Possible Sets of Closing Branches
fn number_of_sets(n: i32, max_distance: i32, roads: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut ans = 0;
    for mask in 0..(1 << n) {
        let mut g = vec![vec![i32::MAX / 4; n]; n];
        for road in &roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            let w = road[2];
            if (mask >> u) & 1 != 0 && (mask >> v) & 1 != 0 {
                g[u][v] = g[u][v].min(w);
                g[v][u] = g[v][u].min(w);
            }
        }
        for k in 0..n {
            if (mask >> k) & 1 != 0 {
                g[k][k] = 0;
                for i in 0..n {
                    for j in 0..n {
                        if g[i][k] + g[k][j] < g[i][j] {
                            g[i][j] = g[i][k] + g[k][j];
                        }
                    }
                }
            }
        }
        let mut ok = true;
        'check: for i in 0..n {
            for j in 0..n {
                if (mask >> i) & 1 != 0 && (mask >> j) & 1 != 0 && g[i][j] > max_distance {
                    ok = false;
                    break 'check;
                }
            }
        }
        if ok {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        number_of_sets(3, 5, vec![vec![0, 1, 2], vec![1, 2, 10], vec![0, 2, 10]])
    );
}

#[cfg(test)]
mod tests {
    use super::number_of_sets;

    #[test]
    fn example_one() {
        assert_eq!(
            number_of_sets(3, 5, vec![vec![0, 1, 2], vec![1, 2, 10], vec![0, 2, 10]]),
            5
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            number_of_sets(3, 5, vec![vec![0, 1, 20], vec![0, 1, 10], vec![1, 2, 2], vec![0, 2, 2]]),
            7
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(number_of_sets(1, 10, vec![]), 2);
    }
}
