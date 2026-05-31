/// LeetCode #1599 - Maximum Profit Of Operating A Centennial Wheel
fn min_operations_max_profit(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32, cost: i32) -> i32 {
    let n = customers.len();
    let base: i32 = customers.iter().enumerate().map(|(i, &c)| if grumpy[i] == 0 { c } else { 0 }).sum();
    let mut win = 0i32;
    for i in 0..minutes.min(n as i32) as usize {
        if grumpy[i] == 1 { win += customers[i]; }
    }
    let mut best = win;
    for i in minutes as usize..n {
        if grumpy[i] == 1 { win += customers[i]; }
        if grumpy[i - minutes as usize] == 1 { win -= customers[i - minutes as usize]; }
        best = best.max(win);
    }
    let profit = base + best;
    if profit <= cost { 0 } else { (profit - cost + minutes - 1) / minutes }
}
fn main() { println!("{}", min_operations_max_profit(vec![1,0,1,0], vec![0,0,0,0], 1, 4)); }
#[cfg(test)]
mod tests {
    use super::min_operations_max_profit;
    #[test]
    fn example_one() { assert_eq!(min_operations_max_profit(vec![1,0,1,0], vec![0,0,0,0], 1, 4), 0); }
}