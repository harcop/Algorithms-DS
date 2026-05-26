/// LeetCode #1399 - Count Largest Group
fn count_largest_group(num: i32) -> i32 {
    use std::collections::HashMap;
    let mut groups: HashMap<i32, i32> = HashMap::new();
    for x in 1..=num {
        let mut n = x;
        let mut s = 0;
        while n > 0 {
            s += n % 10;
            n /= 10;
        }
        *groups.entry(s).or_insert(0) += 1;
    }
    let max_sz = *groups.values().max().unwrap();
    groups.values().filter(|&&c| c == max_sz).count() as i32
}

fn main() {
    println!("{}", count_largest_group(13));
}

#[cfg(test)]
mod tests {
    use super::count_largest_group;

    #[test]
    fn example_one() {
        assert_eq!(count_largest_group(13), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_largest_group(2), 2);
    }
}

