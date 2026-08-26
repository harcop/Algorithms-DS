/// LeetCode #3419 - Minimize the Maximum Edge Weight of Graph
fn min_max_weight(n: i32, edges: Vec<Vec<i32>>, _threshold: i32) -> i32 {
    let n = n as usize;
    let mut rev = vec![Vec::new(); n];
    let mut max_w = 1;
    for e in &edges {
        rev[e[1] as usize].push((e[0] as usize, e[2]));
        max_w = max_w.max(e[2]);
    }
    let reachable = |limit: i32| -> bool {
        let mut seen = vec![false; n];
        let mut stack = vec![0];
        seen[0] = true;
        let mut cnt = 1;
        while let Some(u) = stack.pop() {
            for &(v, w) in &rev[u] {
                if w <= limit && !seen[v] {
                    seen[v] = true;
                    cnt += 1;
                    stack.push(v);
                }
            }
        }
        cnt == n
    };
    let mut lo = 1;
    let mut hi = max_w;
    let mut ans = -1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if reachable(mid) {
            ans = mid;
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        min_max_weight(
            5,
            vec![
                vec![1, 0, 1],
                vec![2, 0, 2],
                vec![3, 0, 1],
                vec![4, 3, 1],
                vec![2, 1, 1]
            ],
            2
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_max_weight;

    #[test]
    fn example1() {
        assert_eq!(
            min_max_weight(
                5,
                vec![
                    vec![1, 0, 1],
                    vec![2, 0, 2],
                    vec![3, 0, 1],
                    vec![4, 3, 1],
                    vec![2, 1, 1]
                ],
                2
            ),
            1
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_max_weight(
                5,
                vec![
                    vec![0, 1, 1],
                    vec![0, 2, 2],
                    vec![0, 3, 1],
                    vec![0, 4, 1],
                    vec![1, 2, 1],
                    vec![1, 4, 1]
                ],
                1
            ),
            -1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            min_max_weight(
                5,
                vec![
                    vec![1, 2, 1],
                    vec![1, 3, 3],
                    vec![1, 4, 5],
                    vec![2, 3, 2],
                    vec![3, 4, 2],
                    vec![4, 0, 1]
                ],
                1
            ),
            2
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            min_max_weight(
                5,
                vec![
                    vec![1, 2, 1],
                    vec![1, 3, 3],
                    vec![1, 4, 5],
                    vec![2, 3, 2],
                    vec![4, 0, 1]
                ],
                1
            ),
            -1
        );
    }
}
