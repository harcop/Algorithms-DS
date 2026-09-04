/// LeetCode #1082 - Sales Analysis I (SQL; Rust analogue)
use std::collections::HashMap;

fn sales_analysis_i(sales: Vec<(i32, i32, i32, String, i32, i32)>) -> Vec<i32> {
    let mut tot: HashMap<i32, i32> = HashMap::new();
    for (seller, _, _, _, _, price) in sales {
        *tot.entry(seller).or_insert(0) += price;
    }
    let mx = tot.values().copied().max().unwrap_or(0);
    let mut ans: Vec<i32> = tot.into_iter().filter(|(_, v)| *v == mx).map(|(s, _)| s).collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::sales_analysis_i;

    #[test]
    fn example() {
        let sales = vec![
            (1, 1, 1, "2019-01-21".into(), 2, 2000),
            (1, 2, 2, "2019-02-17".into(), 1, 800),
            (2, 2, 3, "2019-06-02".into(), 1, 800),
            (3, 3, 4, "2019-05-13".into(), 2, 2800),
        ];
        assert_eq!(sales_analysis_i(sales), vec![1, 3]);
    }
}
