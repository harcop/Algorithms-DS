/// LeetCode #1504 - Count Submatrices With All Ones
fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
    let n = mat.len();
    if n == 0 {
        return 0;
    }
    let m = mat[0].len();
    let mut heights = vec![0; m];
    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            heights[j] = if mat[i][j] == 1 { heights[j] + 1 } else { 0 };
        }
        for left in 0..m {
            let mut min_h = i32::MAX;
            for right in left..m {
                min_h = min_h.min(heights[right]);
                ans += min_h;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", num_submat(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]]));
}

#[cfg(test)]
mod tests {
    use super::num_submat;

    #[test]
    fn example_one() {
        assert_eq!(
            num_submat(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]]),
            13
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(num_submat(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 0]]), 49);
    }
}
