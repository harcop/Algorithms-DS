/// LeetCode #3750 - Minimum Number of Flips to Reverse Binary String
fn minimum_flips(n: i32) -> i32 {
    let s = format!("{:b}", n);
    let m = s.len();
    let b = s.as_bytes();
    let mut cnt = 0;
    for i in 0..m / 2 {
        if b[i] != b[m - i - 1] {
            cnt += 1;
        }
    }
    cnt * 2
}

fn main() {
    println!("{}", minimum_flips(7));
}

#[cfg(test)]
mod tests {
    use super::minimum_flips;

    #[test]
    fn example1() {
        assert_eq!(minimum_flips(7), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_flips(10), 4);
    }
}
