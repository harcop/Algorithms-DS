/// LeetCode #1975 - Maximum Matrix Sum
fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let mut mi = i64::MAX;
    let mut s = 0i64;
    let mut cnt = 0i32;
    for row in matrix {
        for x in row {
            if x < 0 {
                cnt += 1;
            }
            let y = x.abs() as i64;
            mi = mi.min(y);
            s += y;
        }
    }
    if cnt % 2 == 0 {
        s
    } else {
        s - mi * 2
    }
}

fn main() {
    println!("{}", max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]));
}

#[cfg(test)]
mod tests {
    use super::max_matrix_sum;

    #[test]
    fn example_one() {
        assert_eq!(max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_matrix_sum(vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]]),
            16
        );
    }
}
