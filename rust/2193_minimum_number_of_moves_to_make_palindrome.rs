/// LeetCode #2193 - Minimum Number of Moves to Make Palindrome
fn min_moves_to_make_palindrome(s: String) -> i32 {
    let mut cs: Vec<u8> = s.into_bytes();
    let n = cs.len();
    let mut ans = 0i32;
    let mut i = 0usize;
    let mut j = n - 1;

    while i < j {
        let mut even = false;
        for k in (i + 1..=j).rev() {
            if cs[i] == cs[k] {
                even = true;
                let mut k = k;
                while k < j {
                    cs.swap(k, k + 1);
                    k += 1;
                    ans += 1;
                }
                j -= 1;
                break;
            }
        }
        if !even {
            ans += (n / 2 - i) as i32;
        }
        i += 1;
    }

    ans
}

fn main() {
    println!("{}", min_moves_to_make_palindrome("letelt".into()));
}

#[cfg(test)]
mod tests {
    use super::min_moves_to_make_palindrome;

    #[test]
    fn example_one() {
        assert_eq!(min_moves_to_make_palindrome("aabb".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_moves_to_make_palindrome("letelt".into()), 2);
    }
}
