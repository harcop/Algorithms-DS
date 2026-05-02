/// LeetCode #186 - Reverse Words in a String II (in-place char buffer)
fn reverse_words(s: &mut Vec<char>) {
    fn rev_range(a: &mut [char], mut i: usize, mut j: usize) {
        while i < j {
            a.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
    let n = s.len();
    if n == 0 {
        return;
    }
    rev_range(s, 0, n - 1);
    let mut i = 0usize;
    while i < n {
        let mut j = i;
        while j < n && s[j] != ' ' {
            j += 1;
        }
        rev_range(s, i, j.saturating_sub(1));
        i = j + 1;
    }
}

fn main() {
    let mut s: Vec<char> = "the sky is blue".chars().collect();
    reverse_words(&mut s);
    println!("{}", s.iter().collect::<String>());
}

#[cfg(test)]
mod tests {
    use super::reverse_words;

    #[test]
    fn example() {
        let mut s: Vec<char> = "the sky is blue".chars().collect();
        reverse_words(&mut s);
        assert_eq!(s.iter().collect::<String>(), "blue is sky the");
    }
}
