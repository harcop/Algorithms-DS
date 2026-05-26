/// LeetCode #1426 - Counting Elements
fn count_elements(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut cnt = HashMap::new();
    for x in arr {
        *cnt.entry(x).or_insert(0) += 1;
    }
    cnt.values().filter(|&&c| c == 1).count() as i32
}

fn main() {
    println!("{}", count_elements(vec![1, 2, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::count_elements;

    #[test]
    fn example_one() {
        assert_eq!(count_elements(vec![1, 2, 3, 2]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_elements(vec![1, 1, 1, 2]), 1);
    }
}

