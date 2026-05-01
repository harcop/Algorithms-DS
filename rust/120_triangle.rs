/// LeetCode #120 - Triangle
fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
    if triangle.is_empty() {
        return 0;
    }
    for i in (0..triangle.len() - 1).rev() {
        let next = triangle[i + 1].clone();
        for j in 0..triangle[i].len() {
            triangle[i][j] += next[j].min(next[j + 1]);
        }
    }
    triangle[0][0]
}

fn main() {
    println!(
        "{}",
        minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_total;

    #[test]
    fn example_one() {
        assert_eq!(
            minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_total(vec![vec![-10]]), -10);
    }
}
