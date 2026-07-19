/// LeetCode #2500 - Delete Greatest Value in Each Row
fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
    for row in &mut grid {
        row.sort_unstable();
    }
    let mut ans = 0;
    for j in 0..grid[0].len() {
        let mut mx = 0;
        for row in &grid {
            mx = mx.max(row[j]);
        }
        ans += mx;
    }
    ans
}

fn main() {
    println!("{}", delete_greatest_value(vec![vec![1, 2, 4], vec![3, 3, 1]]));
}

#[cfg(test)]
mod tests {
    use super::delete_greatest_value;

    #[test]
    fn example_one() {
        assert_eq!(
            delete_greatest_value(vec![vec![1, 2, 4], vec![3, 3, 1]]),
            8
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(delete_greatest_value(vec![vec![10]]), 10);
    }
}
