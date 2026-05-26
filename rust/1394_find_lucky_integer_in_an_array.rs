/// LeetCode #1394 - Find Lucky Integer In An Array
fn find_lucky(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut cnt = HashMap::new();
    for x in arr {
        *cnt.entry(x).or_insert(0) += 1;
    }
    cnt.into_iter()
        .filter(|(v, c)| *v == *c)
        .map(|(v, _)| v)
        .max()
        .unwrap_or(-1)
}

fn main() {
    println!("{}", find_lucky(vec![2, 2, 2, 4, 4]));
}

#[cfg(test)]
mod tests {
    use super::find_lucky;

    #[test]
    fn example_one() {
        assert_eq!(find_lucky(vec![2, 2, 3, 3, 3]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_lucky(vec![1, 2, 5]), 1);
    }
}

