/// LeetCode #3959 - Check Good Integer
fn check_good_integer(mut n: i32) -> bool {
    let mut s = 0;
    while n > 0 {
        let x = n % 10;
        s += x * (x - 1);
        n /= 10;
    }
    s >= 50
}

fn main() {
    println!("{}", check_good_integer(19));
}

#[cfg(test)]
mod tests {
    use super::check_good_integer;

    #[test]
    fn example1() {
        assert_eq!(check_good_integer(1000), false);
    }

    #[test]
    fn example2() {
        assert_eq!(check_good_integer(19), true);
    }
}
