/// LeetCode #2879 - Display the First Three Rows (Pandas; Rust analogue)
fn display_first_three_rows<T: Clone>(rows: Vec<T>) -> Vec<T> {
    rows.into_iter().take(3).collect()
}

fn main() {
    let employees = vec![
        (3, "Bob", "Operations", 48675),
        (90, "Alice", "Sales", 11096),
        (9, "Tatiana", "Engineering", 33805),
        (60, "Annabelle", "InformationTechnology", 37678),
    ];
    println!("{:?}", display_first_three_rows(employees));
}

#[cfg(test)]
mod tests {
    use super::display_first_three_rows;

    #[test]
    fn example() {
        let employees = vec![
            (3, "Bob", "Operations", 48675),
            (90, "Alice", "Sales", 11096),
            (9, "Tatiana", "Engineering", 33805),
            (60, "Annabelle", "InformationTechnology", 37678),
            (49, "Jonathan", "HumanResources", 23793),
            (43, "Khaled", "Administration", 40454),
        ];
        assert_eq!(
            display_first_three_rows(employees),
            vec![
                (3, "Bob", "Operations", 48675),
                (90, "Alice", "Sales", 11096),
                (9, "Tatiana", "Engineering", 33805),
            ]
        );
    }
}
