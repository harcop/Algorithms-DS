/// LeetCode #1421 - NPV Queries (SQL; Rust analogue)
use std::collections::HashMap;

fn npv_queries(npv: Vec<(i32, i32, i32)>, queries: Vec<(i32, i32)>) -> Vec<(i32, i32, i32)> {
    let map: HashMap<(i32, i32), i32> = npv.into_iter().map(|(id, y, v)| ((id, y), v)).collect();
    queries
        .into_iter()
        .map(|(id, y)| (id, y, *map.get(&(id, y)).unwrap_or(&0)))
        .collect()
}

fn main() {
    println!("{:?}", npv_queries(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::npv_queries;

    #[test]
    fn example() {
        let npv = vec![
            (1, 2018, 100),
            (7, 2020, 30),
            (13, 2019, 40),
            (1, 2019, 113),
            (2, 2008, 121),
            (3, 2009, 12),
            (11, 2020, 99),
            (7, 2019, 0),
        ];
        let queries = vec![
            (1, 2019),
            (2, 2008),
            (3, 2009),
            (7, 2018),
            (7, 2019),
            (7, 2020),
            (13, 2019),
        ];
        assert_eq!(
            npv_queries(npv, queries),
            vec![
                (1, 2019, 113),
                (2, 2008, 121),
                (3, 2009, 12),
                (7, 2018, 0),
                (7, 2019, 0),
                (7, 2020, 30),
                (13, 2019, 40),
            ]
        );
    }
}
