/// LeetCode #54 - Spiral Matrix
fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    let mut top = 0i32;
    let mut bottom = matrix.len() as i32 - 1;
    let mut left = 0i32;
    let mut right = matrix[0].len() as i32 - 1;
    let mut out = Vec::with_capacity(matrix.len() * matrix[0].len());

    while left <= right && top <= bottom {
        for c in left..=right {
            out.push(matrix[top as usize][c as usize]);
        }
        top += 1;

        for r in top..=bottom {
            out.push(matrix[r as usize][right as usize]);
        }
        right -= 1;

        if top <= bottom {
            for c in (left..=right).rev() {
                out.push(matrix[bottom as usize][c as usize]);
            }
            bottom -= 1;
        }

        if left <= right {
            for r in (top..=bottom).rev() {
                out.push(matrix[r as usize][left as usize]);
            }
            left += 1;
        }
    }

    out
}

fn main() {
    println!("{:?}", spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]));
}

#[cfg(test)]
mod tests {
    use super::spiral_order;

    #[test]
    fn example_one() {
        assert_eq!(
            spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            spiral_order(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
