/// LeetCode #2706 - Buy Two Chocolates
fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
    let mut a = 1000;
    let mut b = 1000;
    for x in prices {
        if x < a {
            b = a;
            a = x;
        } else if x < b {
            b = x;
        }
    }
    let cost = a + b;
    if money < cost {
        money
    } else {
        money - cost
    }
}

fn main() {
    println!("{}", buy_choco(vec![1, 2, 2], 3));
}

#[cfg(test)]
mod tests {
    use super::buy_choco;

    #[test]
    fn example_one() {
        assert_eq!(buy_choco(vec![1, 2, 2], 3), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(buy_choco(vec![3, 2, 3], 3), 3);
    }
}
