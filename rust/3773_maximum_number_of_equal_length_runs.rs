/// LeetCode #3773 - Maximum Number of Equal Length Runs (premium)
use std::collections::HashMap;

fn max_same_length_runs(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut cnt: HashMap<usize, i32> = HashMap::new();
    let mut ans = 0;
    let mut i = 0;
    while i < n {
        let mut j = i + 1;
        while j < n && b[j] == b[i] {
            j += 1;
        }
        let e = cnt.entry(j - i).or_insert(0);
        *e += 1;
        ans = ans.max(*e);
        i = j;
    }
    ans
}

fn main() {
    println!("{}", max_same_length_runs("hello".into()));
}

#[cfg(test)]
mod tests {
    use super::max_same_length_runs;

    #[test]
    fn example1() {
        assert_eq!(max_same_length_runs("hello".into()), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_same_length_runs("aaabaaa".into()), 2);
    }
}
