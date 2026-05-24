/// LeetCode #1247 - Minimum Swaps to Make Strings Equal
fn minimum_swap(s: String, t: String) -> i32 {
    let mut xy = 0i32;
    let mut yx = 0i32;
    for (a, b) in s.bytes().zip(t.bytes()) {
        if a == b'x' && b == b'y' {
            xy += 1;
        } else if a == b'y' && b == b'x' {
            yx += 1;
        }
    }
    if (xy + yx) % 2 != 0 {
        return -1;
    }
    xy / 2 + yx / 2 + xy % 2 + yx % 2
}

fn main() {
    println!("{}", minimum_swap("xx".into(), "yy".into()));
}

#[cfg(test)]
mod tests {
    use super::minimum_swap;

    #[test]
    fn example_one() {
        assert_eq!(minimum_swap("xx".into(), "yy".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_swap("xy".into(), "yx".into()), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_swap("xx".into(), "xx".into()), 0);
    }
}
