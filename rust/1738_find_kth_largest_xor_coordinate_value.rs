/// LeetCode #1738 - Find Kth Largest XOR Coordinate Value
fn kth_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut pref = vec![vec![0i32; n + 1]; m + 1];
    for i in 0..m {
        for j in 0..n {
            pref[i + 1][j + 1] =
                pref[i][j + 1] ^ pref[i + 1][j] ^ pref[i][j] ^ matrix[i][j];
        }
    }
    let mut vals = Vec::with_capacity(m * n);
    for i in 1..=m {
        for j in 1..=n {
            vals.push(pref[i][j]);
        }
    }
    vals.sort_unstable();
    vals[vals.len() - k as usize]
}
fn main() {
    println!("{}", kth_value(vec![vec![5, 2], vec![1, 6]], 1));
}
#[cfg(test)]
mod tests {
    use super::kth_value;
    #[test]
    fn example_one() {
        assert_eq!(kth_value(vec![vec![5, 2], vec![1, 6]], 1), 7);
    }
    #[test]
    fn example_two() {
        assert_eq!(
            kth_value(vec![vec![5, 2], vec![1, 6], vec![3, 12]], 3),
            7
        );
    }
}
