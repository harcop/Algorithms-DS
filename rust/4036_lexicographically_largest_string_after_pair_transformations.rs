/// LeetCode #4036 - Lexicographically Largest String After Pair Transformations
fn largest_string(nums: Vec<i32>) -> Vec<String> {
    let mut ans = Vec::new();
    for mut x in nums {
        let mut s = String::new();
        for j in (0..26).rev() {
            let t = x >> j;
            if t > 0 {
                let ch = (b'a' + j as u8) as char;
                for _ in 0..t {
                    s.push(ch);
                }
            }
            x &= (1 << j) - 1;
        }
        ans.push(s);
    }
    ans
}

fn main() {
    println!("{:?}", largest_string(vec![2, 5, 7]));
}

#[cfg(test)]
mod tests {
    use super::largest_string;

    #[test]
    fn example1() {
        assert_eq!(
            largest_string(vec![2, 5, 7]),
            vec!["b".to_string(), "ca".to_string(), "cba".to_string()]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            largest_string(vec![3, 9, 1]),
            vec!["ba".to_string(), "da".to_string(), "a".to_string()]
        );
    }
}
