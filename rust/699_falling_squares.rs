/// LeetCode #699 - Falling Squares (O(n^2) baseline)
fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
    let n = positions.len();
    let mut heights = vec![0i32; n];
    let mut result = Vec::with_capacity(n);
    let mut max_so_far = 0i32;
    for i in 0..n {
        let l1 = positions[i][0];
        let s1 = positions[i][1];
        let r1 = l1 + s1;
        let mut h = s1;
        for j in 0..i {
            let l2 = positions[j][0];
            let s2 = positions[j][1];
            let r2 = l2 + s2;
            if l1 < r2 && l2 < r1 {
                h = h.max(heights[j] + s1);
            }
        }
        heights[i] = h;
        max_so_far = max_so_far.max(h);
        result.push(max_so_far);
    }
    result
}

fn main() {
    println!(
        "{:?}",
        falling_squares(vec![vec![1, 2], vec![2, 3], vec![6, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::falling_squares;

    #[test]
    fn example_one() {
        assert_eq!(
            falling_squares(vec![vec![1, 2], vec![2, 3], vec![6, 1]]),
            vec![2, 5, 5]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            falling_squares(vec![vec![100, 100], vec![200, 100]]),
            vec![100, 100]
        );
    }
}
