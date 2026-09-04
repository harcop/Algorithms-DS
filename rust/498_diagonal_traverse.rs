/// LeetCode #498 - Diagonal Traverse
fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let m = mat.len();
    let n = mat[0].len();
    let mut ans = Vec::with_capacity(m * n);
    let mut i = 0i32;
    let mut j = 0i32;
    let mut up = true;
    for _ in 0..m * n {
        ans.push(mat[i as usize][j as usize]);
        if up {
            if j == n as i32 - 1 {
                i += 1;
                up = false;
            } else if i == 0 {
                j += 1;
                up = false;
            } else {
                i -= 1;
                j += 1;
            }
        } else if i == m as i32 - 1 {
            j += 1;
            up = true;
        } else if j == 0 {
            i += 1;
            up = true;
        } else {
            i += 1;
            j -= 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
}

#[cfg(test)]
mod tests {
    use super::find_diagonal_order;

    #[test]
    fn example_one() {
        assert_eq!(
            find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 4, 7, 5, 3, 6, 8, 9]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_diagonal_order(vec![vec![1, 2], vec![3, 4]]),
            vec![1, 2, 3, 4]
        );
    }
}
