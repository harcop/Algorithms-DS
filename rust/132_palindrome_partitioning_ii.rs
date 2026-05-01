/// LeetCode #132 - Palindrome Partitioning II
fn min_cut(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    if n <= 1 {
        return 0;
    }
    let mut pal = vec![vec![false; n]; n];
    for i in (0..n).rev() {
        for j in i..n {
            pal[i][j] = b[i] == b[j] && (j - i < 2 || pal[i + 1][j - 1]);
        }
    }
    let mut cuts = vec![0i32; n];
    for j in 0..n {
        let mut best = j as i32 + 1;
        for i in 0..=j {
            if pal[i][j] {
                best = if i == 0 {
                    0
                } else {
                    best.min(cuts[i - 1] + 1)
                };
            }
        }
        cuts[j] = best;
    }
    cuts[n - 1]
}

fn main() {
    println!("{}", min_cut("aab".to_string()));
}

#[cfg(test)]
mod tests {
    use super::min_cut;

    #[test]
    fn example_one() {
        assert_eq!(min_cut("aab".to_string()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_cut("a".to_string()), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_cut("ab".to_string()), 1);
    }
}
