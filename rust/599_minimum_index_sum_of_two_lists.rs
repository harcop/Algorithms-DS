/// LeetCode #599 - Minimum Index Sum of Two Lists
use std::collections::HashMap;

fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let mut idx: HashMap<&str, usize> = HashMap::new();
    for (i, s) in list1.iter().enumerate() {
        idx.insert(s.as_str(), i);
    }
    let mut best = usize::MAX;
    let mut out: Vec<String> = vec![];
    for (j, s) in list2.iter().enumerate() {
        if let Some(&i) = idx.get(s.as_str()) {
            let sum = i + j;
            if sum < best {
                best = sum;
                out = vec![s.clone()];
            } else if sum == best {
                out.push(s.clone());
            }
        }
    }
    out
}

fn main() {
    println!(
        "{:?}",
        find_restaurant(
            vec!["Shogun".into(), "Tapioca Express".into(), "Burger King".into(), "KFC".into()],
            vec![
                "Piatti".into(),
                "The Grill at Torrey Pines".into(),
                "Hungry Hunter Steakhouse".into(),
                "Shogun".into()
            ]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::find_restaurant;

    #[test]
    fn example_one() {
        assert_eq!(
            find_restaurant(
                vec!["Shogun".into(), "Tapioca Express".into(), "Burger King".into(), "KFC".into()],
                vec![
                    "Piatti".into(),
                    "The Grill at Torrey Pines".into(),
                    "Hungry Hunter Steakhouse".into(),
                    "Shogun".into()
                ]
            ),
            vec!["Shogun".to_string()]
        );
    }
}
