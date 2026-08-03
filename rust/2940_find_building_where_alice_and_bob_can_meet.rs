/// LeetCode #2940 - Find Building Where Alice and Bob Can Meet
fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = heights.len();
    let mut ans = vec![-1; queries.len()];
    let mut deferred: Vec<Vec<(i32, usize)>> = vec![vec![]; n];

    for (qi, q) in queries.iter().enumerate() {
        let mut a = q[0] as usize;
        let mut b = q[1] as usize;
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        if a == b || heights[a] < heights[b] {
            ans[qi] = b as i32;
        } else {
            deferred[b].push((heights[a], qi));
        }
    }

    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut pq: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();

    for i in 0..n {
        while let Some(Reverse((h, qi))) = pq.peek() {
            if *h >= heights[i] {
                break;
            }
            ans[*qi] = i as i32;
            pq.pop();
        }
        for &(h, qi) in &deferred[i] {
            pq.push(Reverse((h, qi)));
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        leftmost_building_queries(
            vec![6, 4, 8, 5, 2, 7],
            vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::leftmost_building_queries;

    #[test]
    fn example_one() {
        assert_eq!(
            leftmost_building_queries(
                vec![6, 4, 8, 5, 2, 7],
                vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]]
            ),
            vec![2, 5, -1, 5, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            leftmost_building_queries(
                vec![5, 3, 8, 2, 6, 1, 4, 6],
                vec![vec![0, 7], vec![3, 5], vec![5, 2], vec![3, 0], vec![1, 6]]
            ),
            vec![7, 6, -1, 4, 6]
        );
    }
}
