/// LeetCode #1052 - Grumpy Bookstore Owner
fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let minutes = minutes as usize;
    let base: i32 = customers
        .iter()
        .zip(&grumpy)
        .filter(|(_, &g)| g == 0)
        .map(|(&c, _)| c)
        .sum();
    let mut window = 0i32;
    for i in 0..minutes.min(customers.len()) {
        if grumpy[i] == 1 {
            window += customers[i];
        }
    }
    let mut best = window;
    for i in minutes..customers.len() {
        if grumpy[i - minutes] == 1 {
            window -= customers[i - minutes];
        }
        if grumpy[i] == 1 {
            window += customers[i];
        }
        best = best.max(window);
    }
    base + best
}

fn main() {
    println!("{}", max_satisfied(vec![1, 0, 1, 2, 1, 1, 7, 5], vec![0, 1, 0, 1, 0, 1, 0, 0], 3));
}

#[cfg(test)]
mod tests {
    use super::max_satisfied;

    #[test]
    fn example_one() {
        assert_eq!(
            max_satisfied(vec![1, 0, 1, 2, 1, 1, 7, 5], vec![0, 1, 0, 1, 0, 1, 0, 0], 3),
            18
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(max_satisfied(vec![1], vec![0], 1), 1);
    }
}
