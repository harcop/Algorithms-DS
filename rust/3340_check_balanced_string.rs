/// LeetCode #3340 - Check Balanced String
fn is_balanced(num: String) -> bool {
    let mut f = [0i32; 2];
    for (i, c) in num.bytes().enumerate() {
        f[i & 1] += (c - b'0') as i32;
    }
    f[0] == f[1]
}

fn main() {
    println!("{}", is_balanced("1234".into()));
}

#[cfg(test)]
mod tests {
    use super::is_balanced;

    #[test]
    fn example1() {
        assert!(!is_balanced("1234".into()));
    }

    #[test]
    fn example2() {
        assert!(is_balanced("24123".into()));
    }
}
