/// LeetCode #1960 - Maximum Product of the Length of Two Palindromic Substrings
fn max_product(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut hlen = vec![0usize; n];
    let mut center = 0usize;
    let mut right = 0usize;

    for i in 0..n {
        if i < right {
            hlen[i] = (right - i).min(hlen[2 * center - i]);
        }
        while i >= 1 + hlen[i]
            && i + 1 + hlen[i] < n
            && s[i - 1 - hlen[i]] == s[i + 1 + hlen[i]]
        {
            hlen[i] += 1;
        }
        if right < i + hlen[i] {
            center = i;
            right = i + hlen[i];
        }
    }

    let mut prefix = vec![0usize; n];
    let mut suffix = vec![0usize; n];
    for i in 0..n {
        let len = 2 * hlen[i] + 1;
        prefix[i + hlen[i]] = prefix[i + hlen[i]].max(len);
        suffix[i - hlen[i]] = suffix[i - hlen[i]].max(len);
    }

    for i in 1..n {
        prefix[n - 1 - i] = prefix[n - 1 - i].max(prefix[n - i].saturating_sub(2));
        suffix[i] = suffix[i].max(suffix[i - 1].saturating_sub(2));
    }

    for i in 1..n {
        prefix[i] = prefix[i].max(prefix[i - 1]);
        suffix[n - 1 - i] = suffix[n - 1 - i].max(suffix[n - i]);
    }

    (1..n)
        .map(|i| prefix[i - 1] * suffix[i])
        .max()
        .unwrap_or(0) as i32
}

fn main() {
    println!("{}", max_product("ababbb".into()));
}

#[cfg(test)]
mod tests {
    use super::max_product;

    #[test]
    fn example_one() {
        assert_eq!(max_product("ababbb".into()), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_product("zaaaxbbby".into()), 9);
    }
}
