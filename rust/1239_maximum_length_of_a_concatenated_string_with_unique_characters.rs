/// LeetCode #1239 - Maximum Length of a Concatenated String with Unique Characters
fn max_length(arr: Vec<String>) -> i32 {
    let masks: Vec<u32> = arr
        .iter()
        .map(|s| {
            let mut m = 0u32;
            for c in s.bytes() {
                let bit = 1u32 << (c - b'a');
                if m & bit != 0 {
                    return u32::MAX;
                }
                m |= bit;
            }
            m
        })
        .collect();
    let mut best = 0i32;
    fn dfs(i: usize, masks: &[u32], cur: u32, len: i32, best: &mut i32) {
        *best = (*best).max(len);
        for j in i..masks.len() {
            let m = masks[j];
            if m == u32::MAX || cur & m != 0 {
                continue;
            }
            dfs(j + 1, masks, cur | m, len + m.count_ones() as i32, best);
        }
    }
    dfs(0, &masks, 0, 0, &mut best);
    best
}

fn main() {
    println!(
        "{}",
        max_length(vec!["un".into(), "iq".into(), "ue".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::max_length;

    #[test]
    fn example_one() {
        assert_eq!(
            max_length(vec!["un".into(), "iq".into(), "ue".into()]),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_length(vec!["cha".into(), "r".into(), "act".into(), "ers".into()]),
            6
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            max_length(vec!["abcdefghijklmnopqrstuvwxyz".into()]),
            26
        );
    }
}
