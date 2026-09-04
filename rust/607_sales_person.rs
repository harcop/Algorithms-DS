/// LeetCode #607 - Sales Person (SQL; Rust analogue)
use std::collections::HashSet;

fn sales_person(
    salesperson: Vec<(i32, String, i32, i32, String)>,
    company: Vec<(i32, String, String)>,
    orders: Vec<(i32, String, i32, i32, i32)>,
) -> Vec<String> {
    let red: HashSet<i32> = company
        .into_iter()
        .filter(|(_, name, _)| name == "RED")
        .map(|(id, _, _)| id)
        .collect();
    let sold_red: HashSet<i32> = orders
        .into_iter()
        .filter(|(_, _, com, _, _)| red.contains(com))
        .map(|(_, _, _, sid, _)| sid)
        .collect();
    salesperson
        .into_iter()
        .filter(|(id, _, _, _, _)| !sold_red.contains(id))
        .map(|(_, name, _, _, _)| name)
        .collect()
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::sales_person;

    #[test]
    fn example() {
        let salesperson = vec![
            (1, "John".into(), 100000, 6, "4/1/2006".into()),
            (2, "Amy".into(), 12000, 5, "5/1/2010".into()),
            (3, "Mark".into(), 65000, 12, "12/25/2008".into()),
            (4, "Pam".into(), 25000, 25, "1/1/2005".into()),
            (5, "Alex".into(), 5000, 10, "2/3/2007".into()),
        ];
        let company = vec![
            (1, "RED".into(), "Boston".into()),
            (2, "ORANGE".into(), "New York".into()),
            (3, "YELLOW".into(), "Boston".into()),
            (4, "GREEN".into(), "Austin".into()),
        ];
        let orders = vec![
            (1, "1/1/2014".into(), 3, 4, 10000),
            (2, "2/1/2014".into(), 4, 5, 5000),
            (3, "3/1/2014".into(), 1, 1, 50000),
            (4, "4/1/2014".into(), 1, 4, 25000),
        ];
        assert_eq!(
            sales_person(salesperson, company, orders),
            vec!["Amy".to_string(), "Mark".to_string(), "Alex".to_string()]
        );
    }
}
