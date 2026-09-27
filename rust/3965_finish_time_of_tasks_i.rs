/// LeetCode #3965 - Finish Time of Tasks I
fn finish_time(n: i32, edges: Vec<Vec<i32>>, base_time: Vec<i32>) -> i64 {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for e in edges {
        g[e[0] as usize].push(e[1] as usize);
    }
    let base: Vec<i64> = base_time.into_iter().map(|x| x as i64).collect();
    let mut down = vec![0i64; n];
    let mut stack = vec![(0usize, 0u8)];
    while let Some((u, state)) = stack.pop() {
        if state == 0 {
            stack.push((u, 1));
            for &v in &g[u] {
                stack.push((v, 0));
            }
        } else if g[u].is_empty() {
            down[u] = base[u];
        } else {
            let mut earliest = i64::MAX;
            let mut latest = i64::MIN;
            for &v in &g[u] {
                earliest = earliest.min(down[v]);
                latest = latest.max(down[v]);
            }
            down[u] = latest + (latest - earliest) + base[u];
        }
    }
    down[0]
}

fn main() {
    println!(
        "{}",
        finish_time(3, vec![vec![0, 1], vec![1, 2]], vec![9, 5, 3])
    );
}

#[cfg(test)]
mod tests {
    use super::finish_time;

    #[test]
    fn example1() {
        assert_eq!(
            finish_time(3, vec![vec![0, 1], vec![1, 2]], vec![9, 5, 3]),
            17
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            finish_time(3, vec![vec![0, 1], vec![0, 2]], vec![4, 7, 6]),
            12
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            finish_time(4, vec![vec![0, 1], vec![0, 2], vec![2, 3]], vec![5, 8, 2, 1]),
            18
        );
    }
}
