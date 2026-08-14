/// LeetCode #3210 - Find the Encrypted String
fn get_encrypted_string(s: String, k: i32) -> String {
    let cs: Vec<u8> = s.bytes().collect();
    let n = cs.len();
    let k = (k as usize) % n;
    (0..n).map(|i| cs[(i + k) % n] as char).collect()
}

fn main() {
    println!("{}", get_encrypted_string("dart".into(), 3));
}

#[cfg(test)]
mod tests {
    use super::get_encrypted_string;

    #[test]
    fn example1() {
        assert_eq!(get_encrypted_string("dart".into(), 3), "tdar");
    }

    #[test]
    fn example2() {
        assert_eq!(get_encrypted_string("aaa".into(), 1), "aaa");
    }
}
