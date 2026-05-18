/// LeetCode #928 - Smallest String With A's and B's
fn smallest_string(n: i32, mut a: i32, mut b: i32) -> String {
    if a == 0 {
        return "b".repeat(b as usize);
    }
    if b == 0 {
        return "a".repeat(a as usize);
    }
    if a > (n + 1) * b || b > (n + 1) * a {
        return String::new();
    }
    let mut s = String::new();
    while a > 0 || b > 0 {
        if a > b {
            let k = if b > 0 { a.min(n) } else { a.min(n) };
            for _ in 0..k {
                s.push('a');
            }
            a -= k;
            if b > 0 {
                s.push('b');
                b -= 1;
            }
        } else {
            let k = if a > 0 { b.min(n) } else { b.min(n) };
            for _ in 0..k {
                s.push('b');
            }
            b -= k;
            if a > 0 {
                s.push('a');
                a -= 1;
            }
        }
    }
    s
}

fn main() {
    println!("{}", smallest_string(2, 2, 1));
}

#[cfg(test)]
mod tests {
    use super::smallest_string;

    #[test]
    fn example_one() {
        assert_eq!(smallest_string(2, 2, 1), "aab".to_string());
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_string(2, 4, 1), String::new());
    }
}
