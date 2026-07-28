/// LeetCode #2734 - Lexicographically Smallest String After Substring Operation
fn smallest_string(s: String) -> String {
    let mut cs: Vec<char> = s.chars().collect();
    let n = cs.len();
    let mut i = 0;
    while i < n && cs[i] == 'a' {
        i += 1;
    }
    if i == n {
        cs[n - 1] = 'z';
        return cs.into_iter().collect();
    }
    let mut j = i;
    while j < n && cs[j] != 'a' {
        cs[j] = ((cs[j] as u8) - 1) as char;
        j += 1;
    }
    cs.into_iter().collect()
}

fn main() {
    println!("{}", smallest_string("cbabc".into()));
}

#[cfg(test)]
mod tests {
    use super::smallest_string;

    #[test]
    fn example_one() {
        assert_eq!(smallest_string("cbabc".into()), "baabc");
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_string("aa".into()), "az");
    }

    #[test]
    fn example_three() {
        assert_eq!(smallest_string("acbbc".into()), "abaab");
    }

    #[test]
    fn example_four() {
        assert_eq!(smallest_string("leetcode".into()), "kddsbncd");
    }
}
