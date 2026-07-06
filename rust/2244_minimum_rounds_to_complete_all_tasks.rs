/// LeetCode #2244 - Minimum Rounds to Complete All Tasks
use std::collections::HashMap;

fn minimum_rounds(tasks: Vec<i32>) -> i32 {
    let mut count: HashMap<i32, i32> = HashMap::new();
    for task in tasks {
        *count.entry(task).or_insert(0) += 1;
    }

    let mut ans = 0;
    for &freq in count.values() {
        if freq == 1 {
            return -1;
        }
        ans += (freq + 2) / 3;
    }
    ans
}

fn main() {
    println!("{}", minimum_rounds(vec![2, 2, 3, 3, 3, 4, 4, 4, 4, 4]));
}

#[cfg(test)]
mod tests {
    use super::minimum_rounds;

    #[test]
    fn example_one() {
        assert_eq!(minimum_rounds(vec![2, 2, 3, 3, 3, 4, 4, 4, 4, 4]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_rounds(vec![2, 3, 3]), -1);
    }
}
