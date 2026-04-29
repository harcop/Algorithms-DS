/// LeetCode #59 - Spiral Matrix II
fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut m = vec![vec![0; n]; n];
    let (mut left, mut right, mut top, mut bottom) = (0i32, n as i32 - 1, 0i32, n as i32 - 1);
    let mut val = 1;

    while left <= right && top <= bottom {
        for c in left..=right {
            m[top as usize][c as usize] = val;
            val += 1;
        }
        top += 1;
        for r in top..=bottom {
            m[r as usize][right as usize] = val;
            val += 1;
        }
        right -= 1;
        if top <= bottom {
            for c in (left..=right).rev() {
                m[bottom as usize][c as usize] = val;
                val += 1;
            }
            bottom -= 1;
        }
        if left <= right {
            for r in (top..=bottom).rev() {
                m[r as usize][left as usize] = val;
                val += 1;
            }
            left += 1;
        }
    }
    m
}

fn main() {
    println!("{:?}", generate_matrix(3));
}

#[cfg(test)]
mod tests {
    use super::generate_matrix;
    #[test]
    fn example_one() {
        assert_eq!(generate_matrix(3), vec![vec![1,2,3],vec![8,9,4],vec![7,6,5]]);
    }
    #[test]
    fn example_two() {
        assert_eq!(generate_matrix(1), vec![vec![1]]);
    }
}
