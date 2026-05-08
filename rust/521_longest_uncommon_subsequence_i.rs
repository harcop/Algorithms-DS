/// LeetCode #521 - Longest Uncommon Subsequence I
fn find_lu_slength(a: String, b: String) -> i32 {
    if a == b {
        -1
    } else {
        a.len().max(b.len()) as i32
    }
}

fn main() {
    println!("{}", find_lu_slength("aba".into(), "cdc".into()));
}

#[cfg(test)]
mod tests {
    use super::find_lu_slength;

    #[test]
    fn example_one() {
        assert_eq!(find_lu_slength("aba".into(), "cdc".into()), 3);
    }
}
