/// LeetCode #1083 - Sales Analysis II (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn sales_analysis_ii(
    product: Vec<(i32, String, i32)>,
    sales: Vec<(i32, i32, i32, String, i32, i32)>,
) -> Vec<i32> {
    let names: HashMap<i32, String> = product.into_iter().map(|(id, n, _)| (id, n)).collect();
    let mut bought: HashMap<i32, HashSet<String>> = HashMap::new();
    for (_, pid, buyer, _, _, _) in sales {
        if let Some(n) = names.get(&pid) {
            bought.entry(buyer).or_default().insert(n.clone());
        }
    }
    let mut ans: Vec<i32> = bought
        .into_iter()
        .filter(|(_, ps)| ps.contains("S8") && !ps.contains("iPhone"))
        .map(|(b, _)| b)
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::sales_analysis_ii;

    #[test]
    fn example() {
        let product = vec![
            (1, "S8".into(), 1000),
            (2, "G4".into(), 800),
            (3, "iPhone".into(), 1400),
        ];
        let sales = vec![
            (1, 1, 1, "2019-01-21".into(), 2, 2000),
            (1, 2, 2, "2019-02-17".into(), 1, 800),
            (2, 2, 3, "2019-06-02".into(), 1, 800),
            (3, 3, 4, "2019-05-13".into(), 2, 2800),
        ];
        assert_eq!(sales_analysis_ii(product, sales), vec![1]);
    }
}
