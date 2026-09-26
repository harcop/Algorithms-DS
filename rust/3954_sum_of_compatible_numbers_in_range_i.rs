/// LeetCode #3954 - Sum of Compatible Numbers in Range I
fn sum_of_compatible(n: i32, k: i32) -> i32 {
    let mut ans = 0;
    let start = 1.max(n - k);
    for x in start..=n + k {
        if n & x == 0 {
            ans += x;
        }
    }
    ans
}

fn main() {
    println!("{}", sum_of_compatible(2, 3));
}

#[cfg(test)]
mod tests {
    use super::sum_of_compatible;

    #[test]
    fn example1() {
        assert_eq!(sum_of_compatible(2, 3), 10);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_of_compatible(5, 1), 0);
    }
}
