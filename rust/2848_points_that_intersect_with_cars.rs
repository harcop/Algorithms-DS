/// LeetCode #2848 - Points That Intersect With Cars
fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
    let mut covered = [false; 101];
    for interval in nums {
        for point in interval[0]..=interval[1] {
            covered[point as usize] = true;
        }
    }
    covered.into_iter().filter(|&point| point).count() as i32
}

fn main() {
    println!(
        "{}",
        number_of_points(vec![vec![3, 6], vec![1, 5], vec![4, 7]])
    );
}

#[cfg(test)]
mod tests {
    use super::number_of_points;

    #[test]
    fn example_one() {
        assert_eq!(
            number_of_points(vec![vec![3, 6], vec![1, 5], vec![4, 7]]),
            7
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_points(vec![vec![1, 3], vec![5, 8]]), 7);
    }
}
