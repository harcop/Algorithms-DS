/// LeetCode #984 - String Without AAA or BBB
fn str_without3a3b(a: i32, b: i32) -> String {
    let mut out = String::new();
    let mut ca = a;
    let mut cb = b;
    let mut last = 0i8;
    while ca > 0 || cb > 0 {
        if ca > cb {
            if last != 2 && ca >= 2 && cb > 0 {
                out.push('a');
                out.push('a');
                out.push('b');
                ca -= 2;
                cb -= 1;
                last = 1;
            } else {
                out.push('a');
                ca -= 1;
                last = 1;
            }
        } else if cb > ca {
            if last != 1 && cb >= 2 && ca > 0 {
                out.push('b');
                out.push('b');
                out.push('a');
                cb -= 2;
                ca -= 1;
                last = 2;
            } else {
                out.push('b');
                cb -= 1;
                last = 2;
            }
        } else if last == 1 {
            out.push('b');
            cb -= 1;
            last = 2;
        } else {
            out.push('a');
            ca -= 1;
            last = 1;
        }
    }
    out
}

fn main() {
    println!("{}", str_without3a3b(1, 2));
}

#[cfg(test)]
mod tests {
    use super::str_without3a3b;

    #[test]
    fn example_one() {
        let s = str_without3a3b(1, 2);
        assert_eq!(s.len(), 3);
        assert!(!s.contains("aaa"));
        assert!(!s.contains("bbb"));
    }

    #[test]
    fn example_two() {
        let s = str_without3a3b(4, 1);
        assert_eq!(s, "aabaa");
    }
}
