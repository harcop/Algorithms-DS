/// LeetCode #1886 - Determine Whether Matrix Can Be Obtained By Rotation
fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
    let n = mat.len();
    let mut ok = 0b1111u8;
    for i in 0..n {
        for j in 0..n {
            if mat[i][j] != target[i][j] {
                ok &= !0b0001;
            }
            if mat[j][n - 1 - i] != target[i][j] {
                ok &= !0b0010;
            }
            if mat[n - 1 - i][n - 1 - j] != target[i][j] {
                ok &= !0b0100;
            }
            if mat[n - 1 - j][i] != target[i][j] {
                ok &= !0b1000;
            }
            if ok == 0 {
                return false;
            }
        }
    }
    ok != 0
}

fn main() {
    let mat = vec![vec![0, 1], vec![1, 0]];
    let target = vec![vec![1, 0], vec![0, 1]];
    println!("{}", find_rotation(mat, target));
}

#[cfg(test)]
mod tests {
    use super::find_rotation;

    #[test]
    fn example_one() {
        assert!(find_rotation(
            vec![vec![0, 1], vec![1, 0]],
            vec![vec![1, 0], vec![0, 1]]
        ));
    }
}
