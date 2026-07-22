/// LeetCode #2578 - Split With Minimum Sum
fn split_num(num: i32) -> i32 {
    let mut s: Vec<u8> = num.to_string().into_bytes();
    s.sort_unstable();
    let mut ans = [0i32; 2];
    for (i, c) in s.iter().enumerate() {
        ans[i & 1] = ans[i & 1] * 10 + (c - b'0') as i32;
    }
    ans[0] + ans[1]
}

fn main() {
    println!("{}", split_num(4325));
}

#[cfg(test)]
mod tests {
    use super::split_num;

    #[test]
    fn example_one() {
        assert_eq!(split_num(4325), 59);
    }

    #[test]
    fn example_two() {
        assert_eq!(split_num(687), 75);
    }
}
