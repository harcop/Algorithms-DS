/// LeetCode #3863 - Minimum Operations to Sort a String
fn min_operations(s: String) -> i32 {
    let cs = s.as_bytes();
    let n = cs.len();
    let mut sorted = true;
    let mut mn = cs[0];
    let mut mx = cs[0];
    for i in 1..n {
        mn = mn.min(cs[i]);
        mx = mx.max(cs[i]);
        if cs[i] < cs[i - 1] {
            sorted = false;
        }
    }
    if sorted {
        return 0;
    }
    if n == 2 {
        return -1;
    }
    if cs[0] == mn || cs[n - 1] == mx {
        return 1;
    }
    for i in 1..n - 1 {
        if cs[i] == mn || cs[i] == mx {
            return 2;
        }
    }
    3
}

fn main() {
    println!("{}", min_operations("dog".into()));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations("dog".into()), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations("card".into()), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations("gf".into()), -1);
    }
}
