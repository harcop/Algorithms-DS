/// LeetCode #3360 - Stone Removal Game
fn can_alice_win(mut n: i32) -> bool {
    let mut x = 10;
    let mut k = 0;
    while n >= x {
        n -= x;
        x -= 1;
        k += 1;
    }
    k % 2 == 1
}

fn main() {
    println!("{}", can_alice_win(12));
}

#[cfg(test)]
mod tests {
    use super::can_alice_win;

    #[test]
    fn example1() {
        assert!(can_alice_win(12));
    }

    #[test]
    fn example2() {
        assert!(!can_alice_win(1));
    }
}
