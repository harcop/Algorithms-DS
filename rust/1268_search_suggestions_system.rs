/// LeetCode #1268 - Search Suggestions System
fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
    let mut products = products;
    products.sort();
    let mut res = Vec::new();
    let mut prefix = String::new();
    for c in search_word.chars() {
        prefix.push(c);
        let mut list = Vec::new();
        for p in &products {
            if p.starts_with(&prefix) {
                list.push(p.clone());
                if list.len() == 3 {
                    break;
                }
            }
        }
        res.push(list);
    }
    res
}

fn main() {
    println!(
        "{:?}",
        suggested_products(
            vec![
                "mobile".into(),
                "mouse".into(),
                "moneypot".into(),
                "monitor".into(),
                "mousepad".into(),
            ],
            "mouse".into(),
        )
    );
}

#[cfg(test)]
mod tests {
    use super::suggested_products;

    #[test]
    fn example_one() {
        assert_eq!(
            suggested_products(
                vec![
                    "mobile".into(),
                    "mouse".into(),
                    "moneypot".into(),
                    "monitor".into(),
                    "mousepad".into(),
                ],
                "mouse".into(),
            ),
            vec![
                vec![
                    String::from("mobile"),
                    String::from("moneypot"),
                    String::from("monitor"),
                ],
                vec![
                    String::from("mobile"),
                    String::from("moneypot"),
                    String::from("monitor"),
                ],
                vec![String::from("mouse"), String::from("mousepad")],
                vec![String::from("mouse"), String::from("mousepad")],
                vec![String::from("mouse"), String::from("mousepad")],
            ]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            suggested_products(
                vec![
                    "havana".into(),
                    "hat".into(),
                    "haskell".into(),
                    "havana".into(),
                ],
                "havana".into(),
            ),
            vec![
                vec![String::from("havana"), String::from("havana")],
                vec![String::from("havana"), String::from("havana")],
                vec![String::from("havana"), String::from("havana")],
                vec![String::from("havana"), String::from("havana")],
                vec![String::from("havana"), String::from("havana")],
                vec![String::from("havana"), String::from("havana")],
            ]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            suggested_products(vec!["bags".into(), "tat".into()], "bags".into()),
            vec![
                vec![String::from("bags")],
                vec![String::from("bags")],
                vec![String::from("bags")],
                vec![],
            ]
        );
    }
}
