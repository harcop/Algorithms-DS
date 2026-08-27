/// LeetCode #3443 - Maximum Manhattan Distance After K Changes
fn max_distance(s: String, k: i32) -> i32 {
    fn calc(s: &[u8], k: i32, a: u8, b: u8) -> i32 {
        let mut ans = 0;
        let mut mx = 0;
        let mut cnt = 0;
        for &c in s {
            if c == a || c == b {
                mx += 1;
            } else if cnt < k {
                cnt += 1;
                mx += 1;
            } else {
                mx -= 1;
            }
            ans = ans.max(mx);
        }
        ans
    }
    let s = s.as_bytes();
    calc(s, k, b'S', b'E')
        .max(calc(s, k, b'S', b'W'))
        .max(calc(s, k, b'N', b'E'))
        .max(calc(s, k, b'N', b'W'))
}

fn main() {
    println!("{}", max_distance("NWSE".into(), 1));
}

#[cfg(test)]
mod tests {
    use super::max_distance;

    #[test]
    fn example1() {
        assert_eq!(max_distance("NWSE".into(), 1), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_distance("NSWWEW".into(), 3), 6);
    }
}
