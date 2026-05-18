/// LeetCode #1007 - Minimum Domino Rotations For Equal Row
fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
    fn check(target: i32, tops: &[i32], bottoms: &[i32]) -> i32 {
        let mut rot_a = 0i32;
        let mut rot_b = 0i32;
        for i in 0..tops.len() {
            if tops[i] != target && bottoms[i] != target {
                return -1;
            }
            if tops[i] != target {
                rot_a += 1;
            }
            if bottoms[i] != target {
                rot_b += 1;
            }
        }
        rot_a.min(rot_b)
    }
    let a = check(tops[0], &tops, &bottoms);
    let b = check(bottoms[0], &tops, &bottoms);
    if a == -1 && b == -1 { -1 } else { a.min(b) }
}

fn main() {
    println!("{}", min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::min_domino_rotations;

    #[test]
    fn example_one() {
        assert_eq!(
            min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4]), -1);
    }
}
