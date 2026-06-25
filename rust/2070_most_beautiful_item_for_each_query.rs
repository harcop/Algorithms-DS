/// LeetCode #2070 - Most Beautiful Item for Each Query
fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut items = items;
    items.sort_unstable_by_key(|item| item[0]);

    let mut prices = Vec::with_capacity(items.len());
    let mut best = Vec::with_capacity(items.len());
    let mut max_beauty = 0;
    for item in items {
        prices.push(item[0]);
        max_beauty = max_beauty.max(item[1]);
        best.push(max_beauty);
    }

    queries
        .into_iter()
        .map(|q| {
            let idx = prices.partition_point(|&price| price <= q);
            if idx == 0 {
                0
            } else {
                best[idx - 1]
            }
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        maximum_beauty(
            vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
            vec![1, 2, 3, 4, 5, 6],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_beauty;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_beauty(
                vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
                vec![1, 2, 3, 4, 5, 6],
            ),
            vec![2, 4, 5, 5, 6, 6]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_beauty(
                vec![vec![1, 2], vec![1, 2], vec![1, 3], vec![1, 4]],
                vec![1],
            ),
            vec![4]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_beauty(vec![vec![10, 1000]], vec![5]), vec![0]);
    }
}
