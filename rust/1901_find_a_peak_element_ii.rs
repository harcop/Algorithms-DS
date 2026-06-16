/// LeetCode #1901 - Find a Peak Element II
fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut lo = 0usize;
    let mut hi = mat.len() - 1;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let j = mat[mid]
            .iter()
            .enumerate()
            .max_by_key(|(_, &v)| v)
            .map(|(i, _)| i)
            .unwrap();
        if mat[mid][j] > mat[mid + 1][j] {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    let j = mat[lo]
        .iter()
        .enumerate()
        .max_by_key(|(_, &v)| v)
        .map(|(i, _)| i)
        .unwrap();
    vec![lo as i32, j as i32]
}

fn main() {
    println!("{:?}", find_peak_grid(vec![vec![1, 4], vec![3, 2]]));
}

#[cfg(test)]
mod tests {
    use super::find_peak_grid;

    #[test]
    fn example_one() {
        let ans = find_peak_grid(vec![vec![1, 4], vec![3, 2]]);
        assert!(ans == vec![0, 1] || ans == vec![1, 0]);
    }
}
