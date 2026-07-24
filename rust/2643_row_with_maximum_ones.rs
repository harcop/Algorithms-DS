/// LeetCode #2643 - Row With Maximum Ones
fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = vec![0, 0];
    for (i, row) in mat.iter().enumerate() {
        let cnt: i32 = row.iter().sum();
        if ans[1] < cnt {
            ans = vec![i as i32, cnt];
        }
    }
    ans
}

fn main() {
    println!("{:?}", row_and_maximum_ones(vec![vec![0, 1], vec![1, 0]]));
}

#[cfg(test)]
mod tests {
    use super::row_and_maximum_ones;

    #[test]
    fn example_one() {
        assert_eq!(
            row_and_maximum_ones(vec![vec![0, 1], vec![1, 0]]),
            vec![0, 1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            row_and_maximum_ones(vec![vec![0, 0, 0], vec![0, 1, 1]]),
            vec![1, 2]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            row_and_maximum_ones(vec![vec![0, 0], vec![1, 1], vec![0, 0]]),
            vec![1, 2]
        );
    }
}
