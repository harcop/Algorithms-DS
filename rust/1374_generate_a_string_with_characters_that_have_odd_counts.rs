/// LeetCode #1374 - Generate A String With Characters That Have Odd Counts

fn generate_the_string(n: i32, odd: i32) -> String {
    if odd % 2 == 0 {
        return String::new();
    }
    if n < odd {
        return String::new();
    }
    let mut ans = vec!['a'; n as usize];
    let mut remain = n - odd;
    let mut ch = 1i32;
    for i in 0..n as usize {
        if remain > 0 && i % 2 == 0 {
            ans[i] = (b'a' + ch as u8) as char;
            ch += 1;
            remain -= 2;
        }
    }
    ans.into_iter().collect()
}

fn main() {
    println!("{}", generate_the_string(4, 5));
}

#[cfg(test)]
mod tests {
    use super::generate_the_string;

    #[test]
    fn example_one() {
        assert_eq!(generate_the_string(7, 7), "aaaaaaa");
    }

    #[test]
    fn example_two() {
        assert_eq!(generate_the_string(4, 5), "");
    }
}
