/// LeetCode #2399 - Check Distances Between Same Letters
fn check_distances(s: String, distance: Vec<i32>) -> bool {
    let s = s.as_bytes();
    let mut d = [0i32; 26];
    for (i, &b) in s.iter().enumerate() {
        let j = (b - b'a') as usize;
        let i = i as i32;
        if d[j] > 0 && i - d[j] != distance[j] {
            return false;
        }
        d[j] = i + 1;
    }
    true
}

fn main() {
    println!(
        "{}",
        check_distances(
            "abaccb".to_string(),
            vec![1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::check_distances;

    #[test]
    fn example_one() {
        assert!(check_distances(
            "abaccb".to_string(),
            vec![1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ));
    }

    #[test]
    fn example_two() {
        assert!(!check_distances(
            "aa".to_string(),
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ));
    }
}
