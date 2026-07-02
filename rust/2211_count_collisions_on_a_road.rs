/// LeetCode #2211 - Count Collisions on a Road
fn count_collisions(directions: String) -> i32 {
    let dirs = directions.as_bytes();
    let mut left = 0usize;
    let mut right = dirs.len().saturating_sub(1);

    while left < dirs.len() && dirs[left] == b'L' {
        left += 1;
    }
    while right > 0 && dirs[right] == b'R' {
        right -= 1;
    }

    dirs[left..=right]
        .iter()
        .filter(|&&c| c != b'S')
        .count() as i32
}

fn main() {
    println!("{}", count_collisions("RLRSLL".into()));
}

#[cfg(test)]
mod tests {
    use super::count_collisions;

    #[test]
    fn example_one() {
        assert_eq!(count_collisions("RLRSLL".into()), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_collisions("LLRR".into()), 0);
    }
}
