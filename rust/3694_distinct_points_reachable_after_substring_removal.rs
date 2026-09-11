/// LeetCode #3694 - Distinct Points Reachable After Substring Removal
use std::collections::HashSet;

fn distinct_points(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let k = k as usize;
    let mut f = vec![0i32; n + 1];
    let mut g = vec![0i32; n + 1];
    let mut x = 0i32;
    let mut y = 0i32;
    for (i, &c) in s.iter().enumerate() {
        match c {
            b'U' => y += 1,
            b'D' => y -= 1,
            b'L' => x -= 1,
            _ => x += 1,
        }
        f[i + 1] = x;
        g[i + 1] = y;
    }
    let mut st = HashSet::new();
    for i in k..=n {
        let a = f[n] - (f[i] - f[i - k]);
        let b = g[n] - (g[i] - g[i - k]);
        st.insert((a, b));
    }
    st.len() as i32
}

fn main() {
    println!("{}", distinct_points("LUL".to_string(), 1));
}

#[cfg(test)]
mod tests {
    use super::distinct_points;

    #[test]
    fn example1() {
        assert_eq!(distinct_points("LUL".to_string(), 1), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(distinct_points("UDLR".to_string(), 4), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(distinct_points("UU".to_string(), 1), 1);
    }
}
