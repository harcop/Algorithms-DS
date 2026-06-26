/// LeetCode #2093 - Minimum Cost to Reach City With Discounts
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn minimum_cost(n: i32, highways: Vec<Vec<i32>>, discounts: i32) -> i32 {
    let n = n as usize;
    let discounts = discounts as usize;
    let mut g = vec![Vec::<(usize, i32)>::new(); n];
    for e in highways {
        let a = e[0] as usize;
        let b = e[1] as usize;
        let toll = e[2];
        g[a].push((b, toll));
        g[b].push((a, toll));
    }

    let mut dist = vec![vec![i32::MAX; discounts + 1]; n];
    let mut pq = BinaryHeap::new();
    dist[0][0] = 0;
    pq.push(Reverse((0, 0usize, 0usize)));

    while let Some(Reverse((cost, city, used))) = pq.pop() {
        if city == n - 1 {
            return cost;
        }
        if cost != dist[city][used] {
            continue;
        }
        for &(next, toll) in &g[city] {
            let full = cost + toll;
            if full < dist[next][used] {
                dist[next][used] = full;
                pq.push(Reverse((full, next, used)));
            }
            if used < discounts {
                let half = cost + toll / 2;
                if half < dist[next][used + 1] {
                    dist[next][used + 1] = half;
                    pq.push(Reverse((half, next, used + 1)));
                }
            }
        }
    }

    -1
}

fn main() {
    println!(
        "{}",
        minimum_cost(
            5,
            vec![
                vec![0, 1, 4],
                vec![2, 1, 3],
                vec![1, 4, 11],
                vec![3, 2, 3],
                vec![3, 4, 2],
            ],
            1,
        )
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_cost(
                5,
                vec![
                    vec![0, 1, 4],
                    vec![2, 1, 3],
                    vec![1, 4, 11],
                    vec![3, 2, 3],
                    vec![3, 4, 2],
                ],
                1,
            ),
            9
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            minimum_cost(
                4,
                vec![
                    vec![1, 3, 17],
                    vec![1, 2, 7],
                    vec![3, 2, 5],
                    vec![0, 1, 6],
                    vec![3, 0, 20],
                ],
                20,
            ),
            8
        );
    }

    #[test]
    fn disconnected() {
        assert_eq!(minimum_cost(4, vec![vec![0, 1, 3], vec![2, 3, 2]], 0), -1);
    }
}
