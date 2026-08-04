/// LeetCode #2965 - Find Missing and Repeated Values
fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len();
    let mut cnt = vec![0; n * n + 1];
    for row in &grid {
        for &v in row {
            cnt[v as usize] += 1;
        }
    }
    let mut ans = vec![0, 0];
    for i in 1..=n * n {
        if cnt[i] == 2 {
            ans[0] = i as i32;
        }
        if cnt[i] == 0 {
            ans[1] = i as i32;
        }
    }
    ans
}

fn main() {
    println!("{:?}", find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]));
}

#[cfg(test)]
mod tests {
    use super::find_missing_and_repeated_values;

    #[test]
    fn example_one() {
        assert_eq!(
            find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]),
            vec![2, 4]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_missing_and_repeated_values(vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]]),
            vec![9, 5]
        );
    }
}
