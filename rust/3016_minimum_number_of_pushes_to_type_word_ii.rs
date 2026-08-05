/// LeetCode #3016 - Minimum Number of Pushes to Type Word II
fn minimum_pushes(word: String) -> i32 {
    let mut freq = [0usize; 26];
    for b in word.bytes() {
        freq[(b - b'a') as usize] += 1;
    }
    let mut counts: Vec<usize> = freq.into_iter().filter(|&c| c > 0).collect();
    counts.sort_unstable_by(|a, b| b.cmp(a));

    let mut ans = 0;
    for (i, &f) in counts.iter().enumerate() {
        ans += (i / 8 + 1) * f;
    }
    ans as i32
}

fn main() {
    println!("{}", minimum_pushes("abcde".into()));
    println!("{}", minimum_pushes("xyzxyzxyzxyz".into()));
    println!("{}", minimum_pushes("aabbccddeeffgghhiiiiii".into()));
}

#[cfg(test)]
mod tests {
    use super::minimum_pushes;

    #[test]
    fn example_one() {
        assert_eq!(minimum_pushes("abcde".into()), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_pushes("xyzxyzxyzxyz".into()), 12);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_pushes("aabbccddeeffgghhiiiiii".into()), 24);
    }
}
