/// LeetCode #2839 - Check if Strings Can be Made Equal With Operations I
fn can_be_equal(s1: String, s2: String) -> bool {
    let mut count = [[0i32; 26]; 2];
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();
    for i in 0..s1.len() {
        let parity = i & 1;
        count[parity][(s1[i] - b'a') as usize] += 1;
        count[parity][(s2[i] - b'a') as usize] -= 1;
    }
    count.iter().all(|row| row.iter().all(|&value| value == 0))
}

fn main() {
    println!("{}", can_be_equal("abcd".into(), "cdab".into()));
}

#[cfg(test)]
mod tests {
    use super::can_be_equal;

    #[test]
    fn examples() {
        assert!(can_be_equal("abcd".into(), "cdab".into()));
        assert!(!can_be_equal("abcd".into(), "dacb".into()));
    }
}
