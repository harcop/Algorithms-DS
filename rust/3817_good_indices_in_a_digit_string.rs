/// LeetCode #3817 - Good Indices in a Digit String (premium)
fn good_indices(s: String) -> Vec<i32> {
    let bytes = s.as_bytes();
    let mut ans = Vec::new();
    for i in 0..bytes.len() {
        let t = i.to_string();
        let k = t.len();
        if i + 1 < k {
            continue;
        }
        if &bytes[i + 1 - k..i + 1] == t.as_bytes() {
            ans.push(i as i32);
        }
    }
    ans
}

fn main() {
    println!("{:?}", good_indices("0234567890112".into()));
}

#[cfg(test)]
mod tests {
    use super::good_indices;

    #[test]
    fn example1() {
        assert_eq!(good_indices("0234567890112".into()), vec![0, 11, 12]);
    }

    #[test]
    fn example2() {
        assert_eq!(good_indices("01234".into()), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn example3() {
        assert_eq!(good_indices("12345".into()), Vec::<i32>::new());
    }
}
