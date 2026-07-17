/// LeetCode #2456 - Most Popular Video Creator
use std::collections::BTreeMap;

fn most_popular_creator(
    creators: Vec<String>,
    ids: Vec<String>,
    views: Vec<i32>,
) -> Vec<Vec<String>> {
    let mut data: BTreeMap<String, (i64, i32, String)> = BTreeMap::new();

    for ((creator, id), view) in creators.into_iter().zip(ids).zip(views) {
        let entry = data.entry(creator).or_insert_with(|| (0, view, id.clone()));
        entry.0 += view as i64;
        if view > entry.1 || (view == entry.1 && id < entry.2) {
            entry.1 = view;
            entry.2 = id;
        }
    }

    let highest = data.values().map(|entry| entry.0).max().unwrap();
    data.into_iter()
        .filter(|(_, entry)| entry.0 == highest)
        .map(|(creator, entry)| vec![creator, entry.2])
        .collect()
}

fn main() {
    println!(
        "{:?}",
        most_popular_creator(
            vec![
                "alice".to_string(),
                "bob".to_string(),
                "alice".to_string(),
                "chris".to_string()
            ],
            vec![
                "one".to_string(),
                "two".to_string(),
                "three".to_string(),
                "four".to_string()
            ],
            vec![5, 10, 5, 4]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::most_popular_creator;

    #[test]
    fn example_one() {
        assert_eq!(
            most_popular_creator(
                vec![
                    "alice".to_string(),
                    "bob".to_string(),
                    "alice".to_string(),
                    "chris".to_string()
                ],
                vec![
                    "one".to_string(),
                    "two".to_string(),
                    "three".to_string(),
                    "four".to_string()
                ],
                vec![5, 10, 5, 4]
            ),
            vec![
                vec!["alice".to_string(), "one".to_string()],
                vec!["bob".to_string(), "two".to_string()]
            ]
        );
    }

    #[test]
    fn chooses_lexicographically_smallest_id() {
        assert_eq!(
            most_popular_creator(
                vec!["a".to_string(), "a".to_string()],
                vec!["z".to_string(), "b".to_string()],
                vec![5, 5]
            ),
            vec![vec!["a".to_string(), "b".to_string()]]
        );
    }
}
