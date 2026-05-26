/// LeetCode #1428 - Leftmost Column With At Least A One
fn leftmost_column_with_one(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    if n == 0 {
        return -1;
    }
    let m = matrix[0].len();
    let mut ans = m as i32;
    for row in &matrix {
        let mut lo = 0usize;
        let mut hi = m;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if row[mid] == 1 {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        if lo < m {
            ans = ans.min(lo as i32);
        }
    }
    if ans == m as i32 { -1 } else { ans }
}

fn main() {
    println!("{}", leftmost_column_with_one(vec![vec![0, 0], vec![1, 1]]));
}

#[cfg(test)]
mod tests {
    use super::leftmost_column_with_one;

    #[test]
    fn example_one() {
        assert_eq!(leftmost_column_with_one(vec![vec![0, 0], vec![1, 1]]), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(leftmost_column_with_one(vec![vec![0, 0], vec![0, 0]]), -1);
    }
}

