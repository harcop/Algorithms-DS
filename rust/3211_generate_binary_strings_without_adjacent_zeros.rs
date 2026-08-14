/// LeetCode #3211 - Generate Binary Strings Without Adjacent Zeros
fn valid_strings(n: i32) -> Vec<String> {
    let n = n as usize;
    let mut ans = Vec::new();
    let mut t = Vec::new();
    fn dfs(i: usize, n: usize, t: &mut Vec<char>, ans: &mut Vec<String>) {
        if i >= n {
            ans.push(t.iter().collect());
            return;
        }
        for j in 0..2 {
            if j == 1 || i == 0 || t[i - 1] == '1' {
                t.push(char::from_digit(j, 10).unwrap());
                dfs(i + 1, n, t, ans);
                t.pop();
            }
        }
    }
    dfs(0, n, &mut t, &mut ans);
    ans
}

fn main() {
    println!("{:?}", valid_strings(3));
}

#[cfg(test)]
mod tests {
    use super::valid_strings;

    fn sorted(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    #[test]
    fn example1() {
        assert_eq!(
            sorted(valid_strings(3)),
            vec!["010", "011", "101", "110", "111"]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(sorted(valid_strings(1)), vec!["0", "1"]);
    }
}
