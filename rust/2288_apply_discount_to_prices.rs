/// LeetCode #2288 - Apply Discount to Prices
fn discount_prices(sentence: String, discount: i32) -> String {
    let disc = discount as u64;
    sentence
        .split(' ')
        .map(|w| {
            if let Some(rest) = w.strip_prefix('$') {
                if rest.is_empty() || !rest.bytes().all(|b| b.is_ascii_digit()) {
                    return w.to_string();
                }
                let Ok(price) = rest.parse::<u64>() else {
                    return w.to_string();
                };
                let numerator = price.saturating_mul(100u64 - disc);
                let dollars = numerator / 100;
                let cents = numerator % 100;
                format!("${}.{:02}", dollars, cents)
            } else {
                w.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    println!(
        "{}",
        discount_prices(
            "there are $1 $2 and 5$ candies in the shop".to_string(),
            50
        )
    );
}

#[cfg(test)]
mod tests {
    use super::discount_prices;

    #[test]
    fn example_one() {
        assert_eq!(
            discount_prices(
                "there are $1 $2 and 5$ candies in the shop".to_string(),
                50
            ),
            "there are $0.50 $1.00 and 5$ candies in the shop".to_string()
        );
    }

    #[test]
    fn no_price_words() {
        assert_eq!(
            discount_prices("abc $ a$ $12x $001".to_string(), 10),
            "abc $ a$ $12x $0.90".to_string()
        );
    }
}

