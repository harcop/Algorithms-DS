/// LeetCode #2806 - Account Balance After Rounded Purchase
fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
    let mut diff = 100;
    let mut x = 0;
    let mut y = 100;
    while y >= 0 {
        let t = (y - purchase_amount).abs();
        if t < diff {
            diff = t;
            x = y;
        }
        y -= 10;
    }
    100 - x
}

fn main() {
    println!("{}", account_balance_after_purchase(9));
}

#[cfg(test)]
mod tests {
    use super::account_balance_after_purchase;

    #[test]
    fn example_one() {
        assert_eq!(account_balance_after_purchase(9), 90);
    }

    #[test]
    fn example_two() {
        assert_eq!(account_balance_after_purchase(15), 80);
    }

    #[test]
    fn example_three() {
        assert_eq!(account_balance_after_purchase(10), 90);
    }
}
