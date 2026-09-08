/// LeetCode #3626 - Find Stores with Inventory Imbalance (SQL; Rust analogue)
use std::collections::HashMap;

#[derive(Clone)]
struct Inv {
    product: String,
    quantity: i32,
    price: f64,
}

fn find_inventory_imbalance(
    stores: Vec<(i32, String, String)>,
    inventory: Vec<(i32, i32, String, i32, f64)>,
) -> Vec<(i32, String, String, String, String, f64)> {
    let store_map: HashMap<i32, (String, String)> = stores
        .into_iter()
        .map(|(id, name, loc)| (id, (name, loc)))
        .collect();
    let mut by_store: HashMap<i32, Vec<Inv>> = HashMap::new();
    for (_iid, sid, product, quantity, price) in inventory {
        by_store.entry(sid).or_default().push(Inv {
            product,
            quantity,
            price,
        });
    }
    let mut ans = Vec::new();
    for (sid, items) in by_store {
        if items.len() < 3 {
            continue;
        }
        let mut most = items[0].clone();
        let mut cheap = items[0].clone();
        for it in &items[1..] {
            if it.price > most.price || (it.price == most.price && it.quantity > most.quantity) {
                most = it.clone();
            }
            if it.price < cheap.price || (it.price == cheap.price && it.quantity > cheap.quantity) {
                cheap = it.clone();
            }
        }
        if most.quantity < cheap.quantity {
            if let Some((name, loc)) = store_map.get(&sid) {
                let ratio = (cheap.quantity as f64 / most.quantity as f64 * 100.0).round() / 100.0;
                ans.push((
                    sid,
                    name.clone(),
                    loc.clone(),
                    most.product,
                    cheap.product,
                    ratio,
                ));
            }
        }
    }
    ans.sort_by(|a, b| {
        b.5.partial_cmp(&a.5)
            .unwrap()
            .then(a.1.cmp(&b.1))
    });
    ans
}

fn main() {
    println!("{:?}", find_inventory_imbalance(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_inventory_imbalance;

    #[test]
    fn example() {
        let stores = vec![
            (1, "Downtown Tech".into(), "New York".into()),
            (2, "Suburb Mall".into(), "Chicago".into()),
            (3, "City Center".into(), "Los Angeles".into()),
            (4, "Corner Shop".into(), "Miami".into()),
            (5, "Plaza Store".into(), "Seattle".into()),
        ];
        let inventory = vec![
            (1, 1, "Laptop".into(), 5, 999.99),
            (2, 1, "Mouse".into(), 50, 19.99),
            (3, 1, "Keyboard".into(), 25, 79.99),
            (4, 1, "Monitor".into(), 15, 299.99),
            (5, 2, "Phone".into(), 3, 699.99),
            (6, 2, "Charger".into(), 100, 25.99),
            (7, 2, "Case".into(), 75, 15.99),
            (8, 2, "Headphones".into(), 20, 149.99),
            (9, 3, "Tablet".into(), 2, 499.99),
            (10, 3, "Stylus".into(), 80, 29.99),
            (11, 3, "Cover".into(), 60, 39.99),
            (12, 4, "Watch".into(), 10, 299.99),
            (13, 4, "Band".into(), 25, 49.99),
            (14, 5, "Camera".into(), 8, 599.99),
            (15, 5, "Lens".into(), 12, 199.99),
        ];
        let ans = find_inventory_imbalance(stores, inventory);
        assert_eq!(ans.len(), 3);
        assert_eq!(ans[0].0, 3);
        assert!((ans[0].5 - 40.0).abs() < 1e-9);
        assert_eq!(ans[1].0, 2);
        assert_eq!(ans[2].0, 1);
    }
}
