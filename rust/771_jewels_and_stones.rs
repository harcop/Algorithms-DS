/// LeetCode #771 - Jewels and Stones
fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut set = [false; 128];
    for c in jewels.bytes() {
        set[c as usize] = true;
    }
    stones.bytes().filter(|&c| set[c as usize]).count() as i32
}

fn main() {
    println!("{}", num_jewels_in_stones("aA".into(), "aAAbbbb".into()));
}

#[cfg(test)]
mod tests {
    use super::num_jewels_in_stones;

    #[test]
    fn example_one() {
        assert_eq!(num_jewels_in_stones("aA".into(), "aAAbbbb".into()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_jewels_in_stones("z".into(), "ZZ".into()), 0);
    }
}
