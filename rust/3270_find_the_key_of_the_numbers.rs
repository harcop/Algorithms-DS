/// LeetCode #3270 - Find the Key of the Numbers
fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
    let mut ans = 0;
    let mut k = 1;
    for _ in 0..4 {
        let x = (num1 / k % 10).min(num2 / k % 10).min(num3 / k % 10);
        ans += x * k;
        k *= 10;
    }
    ans
}

fn main() {
    println!("{}", generate_key(1, 10, 1000));
}

#[cfg(test)]
mod tests {
    use super::generate_key;

    #[test]
    fn example1() {
        assert_eq!(generate_key(1, 10, 1000), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(generate_key(987, 879, 798), 777);
    }

    #[test]
    fn example3() {
        assert_eq!(generate_key(1, 2, 3), 1);
    }
}
