/// LeetCode #73 - Set Matrix Zeroes
fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut row0 = false;
    let mut col0 = false;

    for j in 0..n {
        if matrix[0][j] == 0 {
            row0 = true;
            break;
        }
    }
    for i in 0..m {
        if matrix[i][0] == 0 {
            col0 = true;
            break;
        }
    }

    for i in 1..m {
        for j in 1..n {
            if matrix[i][j] == 0 {
                matrix[i][0] = 0;
                matrix[0][j] = 0;
            }
        }
    }

    for i in 1..m {
        if matrix[i][0] == 0 {
            for j in 1..n {
                matrix[i][j] = 0;
            }
        }
    }
    for j in 1..n {
        if matrix[0][j] == 0 {
            for i in 1..m {
                matrix[i][j] = 0;
            }
        }
    }

    if row0 {
        for j in 0..n {
            matrix[0][j] = 0;
        }
    }
    if col0 {
        for i in 0..m {
            matrix[i][0] = 0;
        }
    }
}

fn main() {
    let mut m = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    set_zeroes(&mut m);
    println!("{m:?}");
}

#[cfg(test)]
mod tests {
    use super::set_zeroes;

    #[test]
    fn example_one() {
        let mut m = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        set_zeroes(&mut m);
        assert_eq!(m, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    }

    #[test]
    fn example_two() {
        let mut m = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        set_zeroes(&mut m);
        assert_eq!(m, vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]);
    }
}
