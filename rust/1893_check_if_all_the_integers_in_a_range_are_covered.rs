/// LeetCode #1893 - Check if All the Integers in a Range Are Covered
fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    let mut diff = [0i32; 52];
    for r in ranges {
        diff[r[0] as usize] += 1;
        diff[r[1] as usize + 1] -= 1;
    }
    let mut s = 0i32;
    for (i, &x) in diff.iter().enumerate() {
        s += x;
        if s <= 0 && left <= i as i32 && i as i32 <= right {
            return false;
        }
    }
    true
}

fn main() {
    println!(
        "{}",
        is_covered(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 2, 5)
    );
}

#[cfg(test)]
mod tests {
    use super::is_covered;

    #[test]
    fn example_one() {
        assert!(is_covered(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 2, 5));
    }

    #[test]
    fn example_two() {
        assert!(!is_covered(vec![vec![1, 10], vec![10, 20]], 21, 21));
    }
}
