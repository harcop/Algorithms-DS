/// LeetCode #868 - Binary Gap
fn binary_gap(n: i32) -> i32 {
    let mut last = -1;
    let mut ans = 0;
    let mut i = 0;
    let mut x = n;
    while x > 0 {
        if x & 1 == 1 {
            if last != -1 {
                ans = ans.max(i - last);
            }
            last = i;
        }
        x >>= 1;
        i += 1;
    }
    ans
}

fn main() {
    println!("{}", binary_gap(22));
}

#[cfg(test)]
mod tests {
    use super::binary_gap;

    #[test]
    fn example_one() {
        assert_eq!(binary_gap(22), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(binary_gap(8), 0);
    }
}
