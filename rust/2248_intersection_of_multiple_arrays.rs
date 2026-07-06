/// LeetCode #2248 - Intersection of Multiple Arrays
fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut count = vec![0; 1001];
    for row in &nums {
        for &value in row {
            count[value as usize] += 1;
        }
    }

    (1..=1000)
        .filter(|&i| count[i] == nums.len())
        .map(|i| i as i32)
        .collect()
}

fn main() {
    println!(
        "{:?}",
        intersection(vec![vec![3, 1, 2, 4, 5], vec![1, 2, 3, 4], vec![3, 4, 5, 6]])
    );
}

#[cfg(test)]
mod tests {
    use super::intersection;

    #[test]
    fn example_one() {
        assert_eq!(
            intersection(vec![vec![3, 1, 2, 4, 5], vec![1, 2, 3, 4], vec![3, 4, 5, 6]]),
            vec![3, 4]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(intersection(vec![vec![1, 2, 3], vec![4, 5, 6]]), Vec::<i32>::new());
    }
}
