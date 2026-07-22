/// LeetCode #2573 - Find the String with LCP
fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
    let n = lcp.len();
    let mut s = vec!['\0'; n];
    let mut i = 0usize;

    for c in b'a'..=b'z' {
        while i < n && s[i] != '\0' {
            i += 1;
        }
        if i == n {
            break;
        }
        for j in i..n {
            if lcp[i][j] > 0 {
                s[j] = c as char;
            }
        }
    }

    if s.iter().any(|&c| c == '\0') {
        return String::new();
    }

    for i in (0..n).rev() {
        for j in (0..n).rev() {
            if s[i] == s[j] {
                if i == n - 1 || j == n - 1 {
                    if lcp[i][j] != 1 {
                        return String::new();
                    }
                } else if lcp[i][j] != lcp[i + 1][j + 1] + 1 {
                    return String::new();
                }
            } else if lcp[i][j] > 0 {
                return String::new();
            }
        }
    }

    s.into_iter().collect()
}

fn main() {
    println!(
        "{}",
        find_the_string(vec![
            vec![4, 0, 2, 0],
            vec![0, 3, 0, 1],
            vec![2, 0, 2, 0],
            vec![0, 1, 0, 1]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::find_the_string;

    #[test]
    fn example_one() {
        assert_eq!(
            find_the_string(vec![
                vec![4, 0, 2, 0],
                vec![0, 3, 0, 1],
                vec![2, 0, 2, 0],
                vec![0, 1, 0, 1]
            ]),
            "abab"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_the_string(vec![
                vec![4, 3, 2, 1],
                vec![3, 3, 2, 1],
                vec![2, 2, 2, 1],
                vec![1, 1, 1, 1]
            ]),
            "aaaa"
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            find_the_string(vec![
                vec![4, 3, 2, 1],
                vec![3, 3, 2, 1],
                vec![2, 2, 2, 1],
                vec![1, 1, 1, 3]
            ]),
            ""
        );
    }
}
