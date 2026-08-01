use std::collections::HashMap;

/// LeetCode #2857 - Count Pairs of Points With Distance K
fn count_pairs(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut seen = HashMap::<(i32, i32), i32>::new();
    let mut answer = 0;

    for coordinate in coordinates {
        let (x, y) = (coordinate[0], coordinate[1]);
        for x_distance in 0..=k {
            let y_distance = k - x_distance;
            answer += seen
                .get(&(x ^ x_distance, y ^ y_distance))
                .copied()
                .unwrap_or(0);
        }
        *seen.entry((x, y)).or_default() += 1;
    }
    answer
}

fn main() {
    println!(
        "{}",
        count_pairs(vec![vec![1, 2], vec![4, 2], vec![1, 3], vec![5, 2]], 5)
    );
}

#[cfg(test)]
mod tests {
    use super::count_pairs;

    #[test]
    fn example_one() {
        assert_eq!(
            count_pairs(vec![vec![1, 2], vec![4, 2], vec![1, 3], vec![5, 2]], 5),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(count_pairs(vec![vec![1, 3]; 5], 0), 10);
    }
}
