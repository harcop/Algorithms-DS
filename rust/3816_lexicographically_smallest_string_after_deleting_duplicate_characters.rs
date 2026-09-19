/// LeetCode #3816 - Lexicographically Smallest String After Deleting Duplicate Characters
fn lex_smallest_after_deletion(s: String) -> String {
    let bytes = s.into_bytes();
    let mut cnt = [0i32; 26];
    for &c in &bytes {
        cnt[(c - b'a') as usize] += 1;
    }
    let mut stk: Vec<u8> = Vec::new();
    for c in bytes {
        while let Some(&top) = stk.last() {
            if top > c && cnt[(top - b'a') as usize] > 1 {
                cnt[(top - b'a') as usize] -= 1;
                stk.pop();
            } else {
                break;
            }
        }
        stk.push(c);
    }
    while let Some(&top) = stk.last() {
        if cnt[(top - b'a') as usize] > 1 {
            cnt[(top - b'a') as usize] -= 1;
            stk.pop();
        } else {
            break;
        }
    }
    String::from_utf8(stk).unwrap()
}

fn main() {
    println!("{}", lex_smallest_after_deletion("aaccb".into()));
}

#[cfg(test)]
mod tests {
    use super::lex_smallest_after_deletion;

    #[test]
    fn example1() {
        assert_eq!(lex_smallest_after_deletion("aaccb".into()), "aacb");
    }

    #[test]
    fn example2() {
        assert_eq!(lex_smallest_after_deletion("z".into()), "z");
    }
}
