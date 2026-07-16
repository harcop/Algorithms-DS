/// LeetCode #2418 - Sort the People
fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut people: Vec<(i32, String)> = heights.into_iter().zip(names).collect();
    people.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    people.into_iter().map(|(_, name)| name).collect()
}

fn main() {
    println!(
        "{:?}",
        sort_people(
            vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()],
            vec![180, 165, 170]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::sort_people;

    #[test]
    fn example_one() {
        assert_eq!(
            sort_people(
                vec!["Mary".to_string(), "John".to_string(), "Emma".to_string()],
                vec![180, 165, 170]
            ),
            vec!["Mary".to_string(), "Emma".to_string(), "John".to_string()]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            sort_people(
                vec!["Alice".to_string(), "Bob".to_string(), "Bob".to_string()],
                vec![155, 185, 150]
            ),
            vec!["Bob".to_string(), "Alice".to_string(), "Bob".to_string()]
        );
    }
}
