/// LeetCode #2002 - Maximum Product of the Length of Two Palindromic Subsequences
fn max_product(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut pal = vec![true; 1 << n];

    for k in 1..(1 << n) {
        let mut i = 0usize;
        let mut j = n - 1;
        while i < j {
            while i < j && (k >> i & 1) == 0 {
                i += 1;
            }
            while i < j && (k >> j & 1) == 0 {
                j -= 1;
            }
            if i < j {
                if s[i] != s[j] {
                    pal[k] = false;
                    break;
                }
                i += 1;
                j -= 1;
            }
        }
    }

    let mut ans = 0i32;
    for i in 1..(1 << n) {
        if !pal[i] {
            continue;
        }
        let mx = ((1 << n) - 1) ^ i;
        let mut j = mx;
        let a = i.count_ones() as i32;
        while j > 0 {
            if pal[j] {
                let b = j.count_ones() as i32;
                ans = ans.max(a * b);
            }
            j = (j - 1) & mx;
        }
    }
    ans
}

fn main() {
    println!("{}", max_product("leetcodecom".into()));
}

#[cfg(test)]
mod tests {
    use super::max_product;

    #[test]
    fn example_one() {
        assert_eq!(max_product("leetcodecom".into()), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_product("bb".into()), 1);
    }
}
