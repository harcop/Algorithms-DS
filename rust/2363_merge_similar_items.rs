/// LeetCode #2363 - Merge Similar Items
fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut count = [0i32; 1001];
    for item in items1.iter().chain(items2.iter()) {
        count[item[0] as usize] += item[1];
    }
    (1..=1000)
        .filter(|&i| count[i] > 0)
        .map(|i| vec![i as i32, count[i]])
        .collect()
}

fn main() {
    println!(
        "{:?}",
        merge_similar_items(vec![vec![1, 1], vec![4, 5], vec![3, 8]], vec![vec![3, 1], vec![1, 5]])
    );
}

#[cfg(test)]
mod tests {
    use super::merge_similar_items;

    #[test]
    fn example_one() {
        assert_eq!(
            merge_similar_items(vec![vec![1, 1], vec![4, 5], vec![3, 8]], vec![vec![3, 1], vec![1, 5]]),
            vec![vec![1, 6], vec![3, 9], vec![4, 5]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            merge_similar_items(
                vec![vec![1, 1], vec![3, 2], vec![2, 3]],
                vec![vec![2, 1], vec![3, 2], vec![1, 3]]
            ),
            vec![vec![1, 4], vec![2, 4], vec![3, 4]]
        );
    }
}
