/// LeetCode #2303 - Calculate Amount Paid in Taxes
fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
    let mut ans = 0i32;
    let mut prev = 0i32;
    for b in brackets {
        let upper = b[0];
        let percent = b[1];
        ans += (income.min(upper) - prev).max(0) * percent;
        prev = upper;
        if income <= upper {
            break;
        }
    }
    ans as f64 / 100.0
}

fn main() {
    println!(
        "{}",
        calculate_tax(vec![vec![3, 50], vec![7, 10], vec![12, 25]], 10)
    );
}

#[cfg(test)]
mod tests {
    use super::calculate_tax;

    #[test]
    fn example_one() {
        let v = calculate_tax(vec![vec![3, 50], vec![7, 10], vec![12, 25]], 10);
        assert!((v - 2.65).abs() < 1e-5);
    }

    #[test]
    fn example_two() {
        let v = calculate_tax(vec![vec![1, 0], vec![4, 25], vec![5, 50]], 2);
        assert!((v - 0.25).abs() < 1e-5);
    }

    #[test]
    fn example_three() {
        let v = calculate_tax(vec![vec![2, 50]], 0);
        assert!((v - 0.0).abs() < 1e-5);
    }
}
