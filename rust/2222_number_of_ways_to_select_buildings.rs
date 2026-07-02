/// LeetCode #2222 - Number of Ways to Select Buildings
fn number_of_ways(s: String) -> i64 {
    let s = s.as_bytes();
    let mut before = [0i64; 2];
    let mut after = [0i64; 2];
    after[0] = s.iter().filter(|&&c| c == b'0').count() as i64;
    after[1] = s.len() as i64 - after[0];

    let mut ans = 0i64;
    for &c in s {
        let num = (c - b'0') as usize;
        after[num] -= 1;
        if num == 0 {
            ans += before[1] * after[1];
        } else {
            ans += before[0] * after[0];
        }
        before[num] += 1;
    }

    ans
}

fn main() {
    println!("{}", number_of_ways("001101".into()));
}

#[cfg(test)]
mod tests {
    use super::number_of_ways;

    #[test]
    fn example_one() {
        assert_eq!(number_of_ways("001101".into()), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_ways("11100".into()), 0);
    }
}
