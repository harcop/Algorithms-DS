/// LeetCode #3387 - Maximize Amount After Two Days of Conversions
use std::collections::HashMap;

fn max_amount(
    initial_currency: String,
    pairs1: Vec<Vec<String>>,
    rates1: Vec<f64>,
    pairs2: Vec<Vec<String>>,
    rates2: Vec<f64>,
) -> f64 {
    let d1 = build(&pairs1, &rates1, &initial_currency);
    let d2 = build(&pairs2, &rates2, &initial_currency);
    d2.iter()
        .map(|(a, r2)| d1.get(a).copied().unwrap_or(0.0) / r2)
        .fold(0.0, f64::max)
}

fn build(pairs: &[Vec<String>], rates: &[f64], init: &str) -> HashMap<String, f64> {
    let mut g: HashMap<String, Vec<(String, f64)>> = HashMap::new();
    for (p, &r) in pairs.iter().zip(rates.iter()) {
        g.entry(p[0].clone()).or_default().push((p[1].clone(), r));
        g.entry(p[1].clone())
            .or_default()
            .push((p[0].clone(), 1.0 / r));
    }
    let mut d = HashMap::new();
    fn dfs(
        g: &HashMap<String, Vec<(String, f64)>>,
        d: &mut HashMap<String, f64>,
        a: &str,
        v: f64,
    ) {
        d.insert(a.to_string(), v);
        if let Some(nbrs) = g.get(a) {
            for (b, r) in nbrs {
                if !d.contains_key(b) {
                    dfs(g, d, b, v * r);
                }
            }
        }
    }
    dfs(&g, &mut d, init, 1.0);
    d
}

fn main() {
    println!(
        "{}",
        max_amount(
            "EUR".into(),
            vec![
                vec!["EUR".into(), "USD".into()],
                vec!["USD".into(), "JPY".into()]
            ],
            vec![2.0, 3.0],
            vec![
                vec!["JPY".into(), "USD".into()],
                vec!["USD".into(), "CHF".into()],
                vec!["CHF".into(), "EUR".into()]
            ],
            vec![4.0, 5.0, 6.0]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::max_amount;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn example1() {
        let ans = max_amount(
            "EUR".into(),
            vec![
                vec!["EUR".into(), "USD".into()],
                vec!["USD".into(), "JPY".into()],
            ],
            vec![2.0, 3.0],
            vec![
                vec!["JPY".into(), "USD".into()],
                vec!["USD".into(), "CHF".into()],
                vec!["CHF".into(), "EUR".into()],
            ],
            vec![4.0, 5.0, 6.0],
        );
        assert!(approx_eq(ans, 720.0), "got {ans}");
    }

    #[test]
    fn example2() {
        let ans = max_amount(
            "NGN".into(),
            vec![vec!["NGN".into(), "EUR".into()]],
            vec![9.0],
            vec![vec!["NGN".into(), "EUR".into()]],
            vec![6.0],
        );
        assert!(approx_eq(ans, 1.5), "got {ans}");
    }

    #[test]
    fn example3() {
        let ans = max_amount(
            "USD".into(),
            vec![vec!["USD".into(), "EUR".into()]],
            vec![1.0],
            vec![vec!["EUR".into(), "JPY".into()]],
            vec![10.0],
        );
        assert!(approx_eq(ans, 1.0), "got {ans}");
    }
}
