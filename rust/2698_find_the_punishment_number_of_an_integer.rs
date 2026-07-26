/// LeetCode #2698 - Find the Punishment Number of an Integer
fn check(s: &[u8], i: usize, x: i32) -> bool {
    if i >= s.len() {
        return x == 0;
    }
    let mut y = 0i32;
    for j in i..s.len() {
        y = y * 10 + (s[j] - b'0') as i32;
        if y > x {
            break;
        }
        if check(s, j + 1, x - y) {
            return true;
        }
    }
    false
}

fn punishment_number(n: i32) -> i32 {
    let mut ans = 0;
    for i in 1..=n {
        let x = i * i;
        let s = x.to_string();
        if check(s.as_bytes(), 0, i) {
            ans += x;
        }
    }
    ans
}

fn main() {
    println!("{}", punishment_number(10));
}

#[cfg(test)]
mod tests {
    use super::punishment_number;

    #[test]
    fn example_one() {
        assert_eq!(punishment_number(10), 182);
    }

    #[test]
    fn example_two() {
        assert_eq!(punishment_number(37), 1478);
    }
}
