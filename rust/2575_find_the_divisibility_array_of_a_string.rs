/// LeetCode #2575 - Find the Divisibility Array of a String
fn divisibility_array(word: String, m: i32) -> Vec<i32> {
    let m = m as i64;
    let mut x = 0i64;
    word.bytes()
        .map(|c| {
            x = (x * 10 + (c - b'0') as i64) % m;
            if x == 0 {
                1
            } else {
                0
            }
        })
        .collect()
}

fn main() {
    println!("{:?}", divisibility_array("998244353".to_string(), 3));
}

#[cfg(test)]
mod tests {
    use super::divisibility_array;

    #[test]
    fn example_one() {
        assert_eq!(
            divisibility_array("998244353".to_string(), 3),
            vec![1, 1, 0, 0, 0, 1, 1, 0, 0]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            divisibility_array("1010".to_string(), 10),
            vec![0, 1, 0, 1]
        );
    }
}
