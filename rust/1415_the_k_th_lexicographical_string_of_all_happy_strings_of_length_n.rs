/// LeetCode #1415 - The K Th Lexicographical String Of All Happy Strings Of Length N
fn get_happy_string(n: i32, k: i32) -> String {
    let n = n as usize;
    let chars = ['a', 'b', 'c'];
    let mut cur = String::new();
    let mut count = 0i32;

    fn dfs(
        n: usize,
        cur: &mut String,
        chars: &[char; 3],
        k: i32,
        count: &mut i32,
        ans: &mut String,
    ) {
        if *count >= k {
            return;
        }
        if cur.len() == n {
            *count += 1;
            if *count == k {
                *ans = cur.clone();
            }
            return;
        }
        for &c in chars {
            if let Some(last) = cur.chars().last() {
                if last == c {
                    continue;
                }
            }
            cur.push(c);
            dfs(n, cur, chars, k, count, ans);
            cur.pop();
            if *count >= k {
                return;
            }
        }
    }

    let mut ans = String::new();
    dfs(n, &mut cur, &chars, k, &mut count, &mut ans);
    ans
}

fn main() {
    println!("{}", get_happy_string(3, 1));
}

#[cfg(test)]
mod tests {
    use super::get_happy_string;

    #[test]
    fn example_one() {
        assert_eq!(get_happy_string(1, 3), "c");
    }

    #[test]
    fn example_two() {
        assert_eq!(get_happy_string(1, 4), "");
    }
}

