/// LeetCode #1370 - Increasing Decreasing String

fn sort_string(s: String) -> String {
    let mut cnt = [0i32; 26];
    for b in s.bytes() {
        cnt[(b - b'a') as usize] += 1;
    }
    let mut out = String::new();
    let mut inc = true;
    let total: i32 = cnt.iter().sum();
    while out.len() < total as usize {
        if inc {
            for i in 0..26 {
                while cnt[i] > 0 {
                    out.push((b'a' + i as u8) as char);
                    cnt[i] -= 1;
                    if out.len() == total as usize {
                        break;
                    }
                }
            }
        } else {
            for i in (0..26).rev() {
                while cnt[i] > 0 {
                    out.push((b'a' + i as u8) as char);
                    cnt[i] -= 1;
                    if out.len() == total as usize {
                        break;
                    }
                }
            }
        }
        inc = !inc;
    }
    out
}

fn main() {
    println!("{}", sort_string("ccc".into()));
}

#[cfg(test)]
mod tests {
    use super::sort_string;

    #[test]
    fn example_one() {
        assert_eq!(sort_string("ccc".into()), "ccc");
    }

    #[test]
    fn example_two() {
        assert_eq!(sort_string("ggyz".into()), "ggyz");
    }
}
