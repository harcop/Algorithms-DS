/// LeetCode #2033 - Minimum Operations to Make a Uni-Value Grid
fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    let mut nums = Vec::new();
    let m = grid[0][0] % x;
    for row in &grid {
        for &v in row {
            if v % x != m {
                return -1;
            }
            nums.push(v);
        }
    }
    nums.sort_unstable();
    let mid = nums[nums.len() / 2];
    nums.iter().map(|&v| (v - mid).abs() / x).sum()
}

fn main() {
    println!("{}", min_operations(vec![vec![2, 4], vec![6, 8]], 2));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example_one() {
        assert_eq!(min_operations(vec![vec![2, 4], vec![6, 8]], 2), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_operations(vec![vec![1, 5], vec![2, 3]], 1), 5);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_operations(vec![vec![1, 2], vec![3, 4]], 2), -1);
    }
}
