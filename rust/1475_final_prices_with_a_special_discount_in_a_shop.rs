/// LeetCode #1475 - Final Prices With A Special Discount In A Shop
fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let n = prices.len();
    let mut res = prices.clone();
    let mut st = Vec::new();
    for i in (0..n).rev() {
        while st.last().map(|&top| prices[top] > prices[i]).unwrap_or(false) {
            st.pop();
        }
        if let Some(&top) = st.last() { res[i] -= prices[top]; }
        st.push(i);
    }
    res
}
fn main() { println!("{:?}", final_prices(vec![8,4,6,2,3])); }
#[cfg(test)]
mod tests {
    use super::final_prices;
    #[test]
    fn example_one() { assert_eq!(final_prices(vec![8,4,6,2,3]), vec![4,2,4,2,3]); }
    #[test]
    fn example_two() { assert_eq!(final_prices(vec![1,2,3,4,5]), vec![1,2,3,4,5]); }
}