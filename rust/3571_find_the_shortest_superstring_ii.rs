/// LeetCode #3571 - Find the Shortest Superstring II
fn shortest_superstring(s1: String, s2: String) -> String {
    let (mut a, mut b) = (s1, s2);
    if a.len() > b.len() {
        std::mem::swap(&mut a, &mut b);
    }
    if b.contains(&a) {
        return b;
    }
    let m = a.len();
    for i in 0..m {
        if b.starts_with(&a[i..]) {
            return format!("{}{}", &a[..i], b);
        }
        if b.ends_with(&a[..m - i]) {
            return format!("{}{}", b, &a[m - i..]);
        }
    }
    format!("{}{}", a, b)
}

fn main() {
    println!("{}", shortest_superstring("aba".into(), "bab".into()));
}

#[cfg(test)]
mod tests {
    use super::shortest_superstring;

    #[test]
    fn example1() {
        let ans = shortest_superstring("aba".into(), "bab".into());
        assert!(ans.contains("aba") && ans.contains("bab"));
        assert_eq!(ans.len(), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(shortest_superstring("aa".into(), "aaa".into()), "aaa");
    }
}
