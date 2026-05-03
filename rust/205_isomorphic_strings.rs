/// LeetCode #205 - Isomorphic Strings
fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut map_s = [-1i32; 256];
    let mut map_t = [-1i32; 256];
    let sb = s.as_bytes();
    let tb = t.as_bytes();
    for i in 0..sb.len() {
        let si = sb[i] as usize;
        let ti = tb[i] as usize;
        if map_s[si] == -1 && map_t[ti] == -1 {
            map_s[si] = ti as i32;
            map_t[ti] = si as i32;
        } else if map_s[si] != ti as i32 || map_t[ti] != si as i32 {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", is_isomorphic("egg".into(), "add".into()));
}

#[cfg(test)]
mod tests {
    use super::is_isomorphic;

    #[test]
    fn example_one() {
        assert!(is_isomorphic("egg".into(), "add".into()));
    }

    #[test]
    fn example_two() {
        assert!(!is_isomorphic("foo".into(), "bar".into()));
    }

    #[test]
    fn example_three() {
        assert!(is_isomorphic("paper".into(), "title".into()));
    }
}
