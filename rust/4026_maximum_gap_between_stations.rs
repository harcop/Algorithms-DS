/// LeetCode #4026 - Maximum Gap Between Stations
fn maximum_gap(skill: String, station: String) -> i32 {
    let skill = skill.as_bytes();
    let station = station.as_bytes();
    let n = skill.len();
    let m = station.len();
    if n <= 1 {
        return 0;
    }
    let mut suf = vec![0i32; n];
    let mut j = m - 1;
    for i in (1..n).rev() {
        while station[j] != skill[i] {
            j -= 1;
        }
        suf[i] = j as i32;
        j -= 1;
    }
    let mut ans = 0;
    let mut pre = 0usize;
    for i in 0..n - 1 {
        while station[pre] != skill[i] {
            pre += 1;
        }
        ans = ans.max(suf[i + 1] - pre as i32);
        pre += 1;
    }
    ans
}

fn main() {
    println!("{}", maximum_gap("aa".to_string(), "aaaa".to_string()));
}

#[cfg(test)]
mod tests {
    use super::maximum_gap;

    #[test]
    fn example1() {
        assert_eq!(maximum_gap("aa".to_string(), "aaaa".to_string()), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_gap("xyz".to_string(), "xyzz".to_string()), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(maximum_gap("cbc".to_string(), "cbcdbc".to_string()), 4);
    }
}
