/// LeetCode #2110 - Number of Smooth Descent Periods of a Stock
fn get_descent_periods(prices: Vec<i32>) -> i64 {
    let mut run = 0i64;
    let mut ans = 0i64;

    for i in 0..prices.len() {
        if i > 0 && prices[i - 1] - prices[i] == 1 {
            run += 1;
        } else {
            run = 1;
        }
        ans += run;
    }

    ans
}

fn main() {
    println!("{}", get_descent_periods(vec![3, 2, 1, 4]));
}

#[cfg(test)]
mod tests {
    use super::get_descent_periods;

    #[test]
    fn example_one() {
        assert_eq!(get_descent_periods(vec![3, 2, 1, 4]), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_descent_periods(vec![8, 6, 7, 7]), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(get_descent_periods(vec![1]), 1);
    }
}
