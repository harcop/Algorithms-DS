/// LeetCode #2937 - Make Three Strings Equal
fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
    let total = s1.len() + s2.len() + s3.len();
    let n = s1.len().min(s2.len()).min(s3.len());
    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();
    let b3 = s3.as_bytes();
    for i in 0..n {
        if !(b1[i] == b2[i] && b2[i] == b3[i]) {
            return if i == 0 {
                -1
            } else {
                (total - 3 * i) as i32
            };
        }
    }
    (total - 3 * n) as i32
}

fn main() {
    println!(
        "{}",
        find_minimum_operations("abc".into(), "abb".into(), "ab".into())
    );
}

#[cfg(test)]
mod tests {
    use super::find_minimum_operations;

    #[test]
    fn example_one() {
        assert_eq!(
            find_minimum_operations("abc".into(), "abb".into(), "ab".into()),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_minimum_operations("dac".into(), "bac".into(), "cac".into()),
            -1
        );
    }
}
