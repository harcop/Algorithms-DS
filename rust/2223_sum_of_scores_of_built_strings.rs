/// LeetCode #2223 - Sum of Scores of Built Strings
fn sum_scores(s: String) -> i64 {
    let s = s.as_bytes();
    let n = s.len();
    let mut z = vec![0usize; n];
    let mut l = 0usize;
    let mut r = 0usize;

    for i in 1..n {
        if i < r {
            z[i] = (r - i).min(z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }

    z.iter().map(|&x| x as i64).sum::<i64>() + n as i64
}

fn main() {
    println!("{}", sum_scores("babab".into()));
}

#[cfg(test)]
mod tests {
    use super::sum_scores;

    #[test]
    fn example_one() {
        assert_eq!(sum_scores("babab".into()), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(sum_scores("azbazbzaz".into()), 14);
    }
}
