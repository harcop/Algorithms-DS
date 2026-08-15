/// LeetCode #3226 - Number of Bit Changes to Make Two Integers Equal
fn min_changes(n: i32, k: i32) -> i32 {
    if (n & k) != k {
        -1
    } else {
        (n ^ k).count_ones() as i32
    }
}

fn main() {
    println!("{}", min_changes(13, 4));
}

#[cfg(test)]
mod tests {
    use super::min_changes;

    #[test]
    fn example1() {
        assert_eq!(min_changes(13, 4), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_changes(21, 21), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(min_changes(14, 13), -1);
    }
}
