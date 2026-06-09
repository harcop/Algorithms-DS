/// LeetCode #1790 - Check If One String Swap Can Make Strings Equal
fn are_almost_equal(s1: String, s2: String) -> bool {
    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();
    let mut cnt = 0;
    let mut c1 = 0u8;
    let mut c2 = 0u8;
    for i in 0..b1.len() {
        if b1[i] != b2[i] {
            cnt += 1;
            if cnt > 2 || (cnt == 2 && (b1[i] != c2 || b2[i] != c1)) {
                return false;
            }
            c1 = b1[i];
            c2 = b2[i];
        }
    }
    cnt != 1
}

fn main() {
    println!("{}", are_almost_equal("bank".into(), "kanb".into()));
}

#[cfg(test)]
mod tests {
    use super::are_almost_equal;

    #[test]
    fn example_one() {
        assert!(are_almost_equal("bank".into(), "kanb".into()));
    }

    #[test]
    fn example_two() {
        assert!(!are_almost_equal("attack".into(), "defend".into()));
    }

    #[test]
    fn example_three() {
        assert!(are_almost_equal("kelb".into(), "kelb".into()));
    }
}
