/// LeetCode #3922 - Minimum Flips to Make Binary String Coherent
fn min_flips(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let ones = b.iter().filter(|&&c| c == b'1').count() as i32;
    let mut ans = ones.min(n as i32 - ones);
    if ones > 0 {
        ans = ans.min(ones - 1);
    }
    if n >= 3 {
        let mid = b[1..n - 1].iter().filter(|&&c| c == b'1').count() as i32;
        let mut cost = mid;
        if b[0] == b'0' {
            cost += 1;
        }
        if b[n - 1] == b'0' {
            cost += 1;
        }
        ans = ans.min(cost);
    }
    ans
}

fn main() {
    println!("{}", min_flips("1010".into()));
}

#[cfg(test)]
mod tests {
    use super::min_flips;

    #[test]
    fn example1() {
        assert_eq!(min_flips("1010".into()), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_flips("0110".into()), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_flips("1000".into()), 0);
    }
}
