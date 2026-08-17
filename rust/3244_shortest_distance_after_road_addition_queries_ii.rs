/// LeetCode #3244 - Shortest Distance After Road Addition Queries II
fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut nxt: Vec<usize> = (1..n).collect();
    nxt.push(0);
    let mut ans = Vec::new();
    let mut cnt = (n - 1) as i32;
    for q in queries {
        let u = q[0] as usize;
        let v = q[1] as usize;
        if nxt[u] > 0 && nxt[u] < v {
            let mut i = nxt[u];
            while i < v {
                cnt -= 1;
                let ni = nxt[i];
                nxt[i] = 0;
                i = ni;
            }
            nxt[u] = v;
        }
        ans.push(cnt);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        shortest_distance_after_queries(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]])
    );
}

#[cfg(test)]
mod tests {
    use super::shortest_distance_after_queries;

    #[test]
    fn example1() {
        assert_eq!(
            shortest_distance_after_queries(5, vec![vec![2, 4], vec![0, 2], vec![0, 4]]),
            vec![3, 2, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            shortest_distance_after_queries(4, vec![vec![0, 3], vec![0, 2]]),
            vec![1, 1]
        );
    }
}
