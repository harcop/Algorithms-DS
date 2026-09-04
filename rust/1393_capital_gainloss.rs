/// LeetCode #1393 - Capital Gain/Loss (SQL; Rust analogue)
use std::collections::HashMap;

fn capital_gainloss(stocks: Vec<(String, String, i32, i32)>) -> Vec<(String, i32)> {
    let mut gain: HashMap<String, i32> = HashMap::new();
    for (name, op, _, price) in stocks {
        let e = gain.entry(name).or_insert(0);
        if op == "Buy" {
            *e -= price;
        } else {
            *e += price;
        }
    }
    gain.into_iter().collect()
}

fn main() {
    println!("{:?}", capital_gainloss(vec![]));
}

#[cfg(test)]
mod tests {
    use super::capital_gainloss;

    #[test]
    fn example() {
        let stocks = vec![
            ("Leetcode".into(), "Buy".into(), 1, 1000),
            ("Corona Masks".into(), "Buy".into(), 2, 10),
            ("Leetcode".into(), "Sell".into(), 5, 9000),
            ("Handbags".into(), "Buy".into(), 17, 30000),
            ("Corona Masks".into(), "Sell".into(), 3, 1010),
            ("Corona Masks".into(), "Buy".into(), 4, 1000),
            ("Corona Masks".into(), "Sell".into(), 5, 500),
            ("Corona Masks".into(), "Buy".into(), 6, 1000),
            ("Handbags".into(), "Sell".into(), 29, 7000),
            ("Corona Masks".into(), "Sell".into(), 10, 10000),
        ];
        let mut got = capital_gainloss(stocks);
        got.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(
            got,
            vec![
                ("Corona Masks".into(), 9500),
                ("Handbags".into(), -23000),
                ("Leetcode".into(), 8000),
            ]
        );
    }
}
