/// LeetCode #1084 - Sales Analysis III (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn sales_analysis_iii(
    product: Vec<(i32, String, i32)>,
    sales: Vec<(i32, i32, i32, String, i32, i32)>,
) -> Vec<(i32, String)> {
    let names: HashMap<i32, String> = product.into_iter().map(|(id, n, _)| (id, n)).collect();
    let mut dates: HashMap<i32, HashSet<String>> = HashMap::new();
    for (_, pid, _, date, _, _) in sales {
        dates.entry(pid).or_default().insert(date);
    }
    let mut ans = Vec::new();
    for (pid, ds) in dates {
        if ds.iter().all(|d| d.as_str() >= "2019-01-01" && d.as_str() <= "2019-03-31") {
            ans.push((pid, names[&pid].clone()));
        }
    }
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::sales_analysis_iii;

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
        assert_eq!(sales_analysis_iii(product, sales), vec![(1, "S8".into())]);
    }
}
