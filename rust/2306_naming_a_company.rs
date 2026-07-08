/// LeetCode #2306 - Naming a Company
use std::collections::HashSet;

fn distinct_names(ideas: Vec<String>) -> i64 {
    let s: HashSet<&str> = ideas.iter().map(|x| x.as_str()).collect();
    let mut f = [[0i64; 26]; 26];

    for v in &ideas {
        let bytes = v.as_bytes();
        let i = (bytes[0] - b'a') as usize;
        let mut t = bytes.to_vec();
        for j in 0..26 {
            t[0] = b'a' + j as u8;
            let candidate = std::str::from_utf8(&t).unwrap();
            if !s.contains(candidate) {
                f[i][j] += 1;
            }
        }
    }

    let mut ans = 0i64;
    for v in &ideas {
        let bytes = v.as_bytes();
        let i = (bytes[0] - b'a') as usize;
        let mut t = bytes.to_vec();
        for j in 0..26 {
            t[0] = b'a' + j as u8;
            let candidate = std::str::from_utf8(&t).unwrap();
            if !s.contains(candidate) {
                ans += f[j][i];
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        distinct_names(vec![
            "coffee".to_string(),
            "donuts".to_string(),
            "time".to_string(),
            "toffee".to_string()
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::distinct_names;

    #[test]
    fn example_one() {
        assert_eq!(
            distinct_names(vec![
                "coffee".to_string(),
                "donuts".to_string(),
                "time".to_string(),
                "toffee".to_string()
            ]),
            6
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            distinct_names(vec!["lack".to_string(), "back".to_string()]),
            0
        );
    }
}
