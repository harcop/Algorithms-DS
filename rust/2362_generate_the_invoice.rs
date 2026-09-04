/// LeetCode #2362 - Generate the Invoice (SQL; Rust analogue)
use std::collections::HashMap;

fn generate_the_invoice(
    products: Vec<(i32, i32)>,
    purchases: Vec<(i32, i32, i32)>,
) -> Vec<(i32, i32, i32)> {
    let price: HashMap<i32, i32> = products.into_iter().collect();
    let mut totals: HashMap<i32, i32> = HashMap::new();
    let mut lines: HashMap<i32, Vec<(i32, i32, i32)>> = HashMap::new();
    for (invoice_id, product_id, quantity) in purchases {
        let p = *price.get(&product_id).unwrap_or(&0);
        let line = quantity * p;
        *totals.entry(invoice_id).or_insert(0) += line;
        lines
            .entry(invoice_id)
            .or_default()
            .push((product_id, quantity, line));
    }
    let best = totals
        .into_iter()
        .min_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)))
        .map(|(id, _)| id);
    let mut ans = match best {
        Some(id) => lines.remove(&id).unwrap_or_default(),
        None => Vec::new(),
    };
    ans.sort();
    ans
}

fn main() {
    println!("{:?}", generate_the_invoice(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::generate_the_invoice;

    #[test]
    fn example_one() {
        let products = vec![(1, 100), (2, 200)];
        let purchases = vec![(1, 1, 2), (3, 2, 1), (2, 2, 3), (2, 1, 4), (4, 1, 10)];
        assert_eq!(
            generate_the_invoice(products, purchases),
            vec![(1, 4, 400), (2, 3, 600)]
        );
    }
}
