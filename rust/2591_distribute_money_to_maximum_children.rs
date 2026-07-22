/// LeetCode #2591 - Distribute Money to Maximum Children
fn dist_money(money: i32, children: i32) -> i32 {
    if money < children {
        return -1;
    }
    if money > 8 * children {
        return children - 1;
    }
    if money == 8 * children - 4 {
        return children - 2;
    }
    (money - children) / 7
}

fn main() {
    println!("{}", dist_money(20, 3));
}

#[cfg(test)]
mod tests {
    use super::dist_money;

    #[test]
    fn example_one() {
        assert_eq!(dist_money(20, 3), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(dist_money(16, 2), 2);
    }
}
