/// LeetCode #394 - Decode String
fn decode_string(s: String) -> String {
    let mut num_stack: Vec<i32> = vec![];
    let mut str_stack: Vec<String> = vec![];
    let mut cur = String::new();
    let mut k = 0i32;
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            k = k * 10 + (ch as i32 - '0' as i32);
        } else if ch == '[' {
            num_stack.push(k);
            str_stack.push(cur);
            cur = String::new();
            k = 0;
        } else if ch == ']' {
            let times = num_stack.pop().unwrap();
            let prev = str_stack.pop().unwrap();
            let mut tmp = String::with_capacity(prev.len() + cur.len() * times as usize);
            tmp.push_str(&prev);
            for _ in 0..times {
                tmp.push_str(&cur);
            }
            cur = tmp;
        } else {
            cur.push(ch);
        }
    }
    cur
}

fn main() {
    println!("{}", decode_string("3[a]2[bc]".into()));
}

#[cfg(test)]
mod tests {
    use super::decode_string;

    #[test]
    fn example_one() {
        assert_eq!(decode_string("3[a]2[bc]".into()), "aaabcbc");
    }
}
