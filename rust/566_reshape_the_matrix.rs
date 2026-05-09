/// LeetCode #566 - Reshape the Matrix
fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let m = mat.len();
    let n = mat[0].len();
    let r = r as usize;
    let c = c as usize;
    if m * n != r * c {
        return mat;
    }
    let mut out = vec![vec![0; c]; r];
    for i in 0..m * n {
        out[i / c][i % c] = mat[i / n][i % n];
    }
    out
}

fn main() {
    println!("{:?}", matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4));
}

#[cfg(test)]
mod tests {
    use super::matrix_reshape;

    #[test]
    fn example_one() {
        assert_eq!(
            matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
            vec![vec![1, 2, 3, 4]]
        );
    }
}
