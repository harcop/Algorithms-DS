/// LeetCode #458 - Poor Pigs
fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
    if buckets <= 1 {
        return 0;
    }
    let base = minutes_to_test / minutes_to_die + 1;
    let b = buckets as i64;
    let mut x = 0i32;
    let mut pow = 1i64;
    while pow < b {
        pow *= base as i64;
        x += 1;
    }
    x
}

fn main() {
    println!("{}", poor_pigs(1000, 15, 60));
}

#[cfg(test)]
mod tests {
    use super::poor_pigs;

    #[test]
    fn example_one() {
        assert_eq!(poor_pigs(1000, 15, 60), 5);
    }
}
