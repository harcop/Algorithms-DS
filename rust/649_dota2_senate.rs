/// LeetCode #649 - Dota2 Senate
use std::collections::VecDeque;

fn predict_party_victory(senate: String) -> String {
    let mut rs: VecDeque<usize> = VecDeque::new();
    let mut ds: VecDeque<usize> = VecDeque::new();
    let bytes = senate.as_bytes();
    let n = bytes.len();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'R' { rs.push_back(i); } else { ds.push_back(i); }
    }
    while !rs.is_empty() && !ds.is_empty() {
        let r = rs.pop_front().unwrap();
        let d = ds.pop_front().unwrap();
        if r < d { rs.push_back(r + n); } else { ds.push_back(d + n); }
    }
    if rs.is_empty() { "Dire".into() } else { "Radiant".into() }
}

fn main() {
    println!("{}", predict_party_victory("RD".into()));
}

#[cfg(test)]
mod tests {
    use super::predict_party_victory;

    #[test]
    fn example_one() {
        assert_eq!(predict_party_victory("RD".into()), "Radiant");
    }

    #[test]
    fn example_two() {
        assert_eq!(predict_party_victory("RDD".into()), "Dire");
    }
}
