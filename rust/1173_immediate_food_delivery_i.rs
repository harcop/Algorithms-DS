/// LeetCode #1173 - Immediate Food Delivery I (SQL; Rust analogue)

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

fn immediate_food_delivery_i(delivery: Vec<(i32, i32, String, String)>) -> f64 {
    if delivery.is_empty() {
        return 0.0;
    }
    let imm = delivery
        .iter()
        .filter(|(_, _, o, p)| o == p)
        .count() as f64;
    round2(imm / delivery.len() as f64 * 100.0)
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::immediate_food_delivery_i;

    #[test]
    fn example() {
        let delivery = vec![
            (1, 1, "2019-08-01".into(), "2019-08-02".into()),
            (2, 5, "2019-08-02".into(), "2019-08-02".into()),
            (3, 1, "2019-08-11".into(), "2019-08-11".into()),
            (4, 3, "2019-08-24".into(), "2019-08-26".into()),
            (5, 4, "2019-08-21".into(), "2019-08-22".into()),
            (6, 2, "2019-08-11".into(), "2019-08-13".into()),
        ];
        assert!((immediate_food_delivery_i(delivery) - 33.33).abs() < 1e-9);
    }
}
