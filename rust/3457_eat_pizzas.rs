/// LeetCode #3457 - Eat Pizzas!
fn max_weight(mut pizzas: Vec<i32>) -> i64 {
    let days = pizzas.len() / 4;
    pizzas.sort_unstable();
    let odd = (days + 1) / 2;
    let even = days - odd;
    let n = pizzas.len();
    let mut ans: i64 = pizzas[n - odd..].iter().map(|&x| x as i64).sum();
    let mut i = n - odd - 2;
    for _ in 0..even {
        ans += pizzas[i] as i64;
        i -= 2;
    }
    ans
}

fn main() {
    println!("{}", max_weight(vec![1, 2, 3, 4, 5, 6, 7, 8]));
}

#[cfg(test)]
mod tests {
    use super::max_weight;

    #[test]
    fn example1() {
        assert_eq!(max_weight(vec![1, 2, 3, 4, 5, 6, 7, 8]), 14);
    }

    #[test]
    fn example2() {
        assert_eq!(max_weight(vec![2, 1, 1, 1, 1, 1, 1, 1]), 3);
    }
}
