/// LeetCode #311 - Sparse Matrix Multiplication
fn multiply(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = mat1.len();
    let k = mat1[0].len();
    let n = mat2[0].len();
    let mut out = vec![vec![0; n]; m];
    for i in 0..m {
        for p in 0..k {
            if mat1[i][p] == 0 {
                continue;
            }
            for j in 0..n {
                out[i][j] += mat1[i][p] * mat2[p][j];
            }
        }
    }
    out
}

fn main() {
    println!("{:?}", multiply(vec![vec![1, 0, 0], vec![-1, 0, 3]], vec![vec![7, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]));
}

#[cfg(test)]
mod tests {
    use super::multiply;

    #[test]
    fn example_one() {
        let a = vec![vec![1, 0, 0], vec![-1, 0, 3]];
        let b = vec![vec![7, 0, 0], vec![0, 0, 0], vec![0, 0, 1]];
        assert_eq!(multiply(a, b), vec![vec![7, 0, 0], vec![-7, 0, 3]]);
    }
}
