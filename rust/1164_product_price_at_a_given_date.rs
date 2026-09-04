/// LeetCode #1164 - Product Price at a Given Date (SQL; Rust analogue)
use std::collections::HashMap;

fn product_price(products: Vec<(i32, i32, String)>) -> Vec<(i32, i32)> {
    let target = "2019-08-16";
    let mut last: HashMap<i32, (String, i32)> = HashMap::new();
    let mut ids = std::collections::HashSet::new();
    for (pid, price, date) in products {
        ids.insert(pid);
        if date.as_str() <= target {
            last.entry(pid)
                .and_modify(|e| {
                    if date > e.0 {
                        *e = (date.clone(), price);
                    }
                })
                .or_insert((date, price));
        }
    }
    let mut ans: Vec<(i32, i32)> = ids
        .into_iter()
        .map(|id| (id, last.get(&id).map(|e| e.1).unwrap_or(10)))
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::product_price;

    #[test]
    fn example() {
        let products = vec![
            (1, 20, "2019-08-14".into()),
            (2, 50, "2019-08-14".into()),
            (1, 30, "2019-08-15".into()),
            (1, 35, "2019-08-16".into()),
            (2, 65, "2019-08-17".into()),
            (3, 20, "2019-08-18".into()),
        ];
        assert_eq!(product_price(products), vec![(1, 35), (2, 50), (3, 10)]);
    }
}
