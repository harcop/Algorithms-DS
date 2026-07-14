/// LeetCode #2391 - Minimum Amount of Time to Collect Garbage
use std::collections::HashMap;

fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    let mut last = HashMap::new();
    let mut ans = 0;
    for (i, s) in garbage.iter().enumerate() {
        ans += s.len() as i32;
        for c in s.chars() {
            last.insert(c, i);
        }
    }
    let mut ts = 0;
    for (i, t) in travel.iter().enumerate() {
        ts += t;
        let house = i + 1;
        for &j in last.values() {
            if j == house {
                ans += ts;
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        garbage_collection(
            vec![
                "G".to_string(),
                "P".to_string(),
                "GP".to_string(),
                "GG".to_string()
            ],
            vec![2, 4, 3]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::garbage_collection;

    #[test]
    fn example_one() {
        assert_eq!(
            garbage_collection(
                vec![
                    "G".to_string(),
                    "P".to_string(),
                    "GP".to_string(),
                    "GG".to_string()
                ],
                vec![2, 4, 3]
            ),
            21
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            garbage_collection(
                vec!["MMM".to_string(), "PGM".to_string(), "GP".to_string()],
                vec![3, 10]
            ),
            37
        );
    }
}
