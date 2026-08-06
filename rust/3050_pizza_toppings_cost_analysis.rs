/// LeetCode #3050 - Pizza Toppings Cost Analysis (SQL; Rust analogue)
fn pizza_toppings_cost_analysis(toppings: Vec<(String, f64)>) -> Vec<(String, f64)> {
    let n = toppings.len();
    let mut result = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                let mut names = vec![
                    toppings[i].0.clone(),
                    toppings[j].0.clone(),
                    toppings[k].0.clone(),
                ];
                names.sort();
                let pizza = names.join(",");
                let cost_cents =
                    ((toppings[i].1 + toppings[j].1 + toppings[k].1) * 100.0).round() as i32;
                result.push((pizza, cost_cents));
            }
        }
    }

    result.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    result
        .into_iter()
        .map(|(name, cents)| (name, cents as f64 / 100.0))
        .collect()
}

fn main() {
    let toppings = vec![
        ("Pepperoni".into(), 0.50),
        ("Sausage".into(), 0.70),
        ("Chicken".into(), 0.55),
        ("Extra Cheese".into(), 0.40),
    ];
    println!("{:?}", pizza_toppings_cost_analysis(toppings));
}

#[cfg(test)]
mod tests {
    use super::pizza_toppings_cost_analysis;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a * 100.0).round() == (b * 100.0).round()
    }

    #[test]
    fn example() {
        let toppings = vec![
            ("Pepperoni".into(), 0.50),
            ("Sausage".into(), 0.70),
            ("Chicken".into(), 0.55),
            ("Extra Cheese".into(), 0.40),
        ];
        let result = pizza_toppings_cost_analysis(toppings);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].0, "Chicken,Pepperoni,Sausage");
        assert!(approx_eq(result[0].1, 1.75));
        assert_eq!(result[1].0, "Chicken,Extra Cheese,Sausage");
        assert!(approx_eq(result[1].1, 1.65));
        assert_eq!(result[2].0, "Extra Cheese,Pepperoni,Sausage");
        assert!(approx_eq(result[2].1, 1.60));
        assert_eq!(result[3].0, "Chicken,Extra Cheese,Pepperoni");
        assert!(approx_eq(result[3].1, 1.45));
    }
}
