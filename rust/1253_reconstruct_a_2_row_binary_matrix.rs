/// LeetCode #1253 - Reconstruct a 2-Row Binary Matrix
fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
    let n = colsum.len();
    let mut top = vec![0i32; n];
    let mut bot = vec![0i32; n];
    let mut u = upper;
    let mut l = lower;
    for (i, &c) in colsum.iter().enumerate() {
        if c == 2 {
            if u == 0 || l == 0 {
                return vec![];
            }
            top[i] = 1;
            bot[i] = 1;
            u -= 1;
            l -= 1;
        }
    }
    for (i, &c) in colsum.iter().enumerate() {
        if c == 1 {
            if u > 0 {
                top[i] = 1;
                u -= 1;
            } else if l > 0 {
                bot[i] = 1;
                l -= 1;
            } else {
                return vec![];
            }
        }
    }
    if u != 0 || l != 0 {
        return vec![];
    }
    vec![top, bot]
}

fn main() {
    println!("{:?}", reconstruct_matrix(2, 1, vec![1, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::reconstruct_matrix;

    #[test]
    fn example_one() {
        assert_eq!(
            reconstruct_matrix(2, 1, vec![1, 1, 1]),
            vec![vec![1, 0, 1], vec![0, 1, 0]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            reconstruct_matrix(2, 3, vec![2, 2, 1, 1]),
            vec![vec![1, 1, 0, 1], vec![1, 1, 1, 0]]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            reconstruct_matrix(5, 5, vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1]),
            vec![]
        );
    }
}
