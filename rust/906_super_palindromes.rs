/// LeetCode #906 - Super Palindromes
fn superpalindromes_in_range(left: String, right: String) -> i32 {
    fn is_pal(x: i128) -> bool {
        if x < 0 {
            return false;
        }
        if x < 10 {
            return true;
        }
        let mut y = 0i128;
        let orig = x;
        let mut tmp = x;
        while tmp > 0 {
            y = y * 10 + tmp % 10;
            tmp /= 10;
        }
        y == orig
    }
    fn make_pal(mut k: i128, odd: bool) -> i128 {
        let mut res = k;
        if odd {
            k /= 10;
        }
        while k > 0 {
            res = res * 10 + k % 10;
            k /= 10;
        }
        res
    }

    let l: i128 = left.parse().unwrap();
    let r: i128 = right.parse().unwrap();
    let mut ans = 0i32;
    let mut k: i128 = 1;
    while k < 200_000 {
        for &odd in &[false, true] {
            let p = make_pal(k, odd);
            if p == 0 {
                continue;
            }
            let sq = p * p;
            if sq > r {
                continue;
            }
            if sq >= l && is_pal(sq) {
                ans += 1;
            }
        }
        k += 1;
    }
    ans
}

fn main() {
    println!(
        "{}",
        superpalindromes_in_range("4".into(), "1000".into())
    );
}

#[cfg(test)]
mod tests {
    use super::superpalindromes_in_range;

    #[test]
    fn example_one() {
        assert_eq!(superpalindromes_in_range("4".into(), "1000".into()), 4);
    }
}
