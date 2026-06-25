/// LeetCode #2086 - Minimum Number of Buckets Required to Collect Rainwater from Houses
fn minimum_buckets(street: String) -> i32 {
    let mut street: Vec<u8> = street.into_bytes();
    let mut buckets = 0;

    for i in 0..street.len() {
        if street[i] != b'H' {
            continue;
        }
        if i > 0 && street[i - 1] == b'B' {
            continue;
        }
        if i + 1 < street.len() && street[i + 1] == b'.' {
            street[i + 1] = b'B';
            buckets += 1;
        } else if i > 0 && street[i - 1] == b'.' {
            street[i - 1] = b'B';
            buckets += 1;
        } else {
            return -1;
        }
    }

    buckets
}

fn main() {
    println!("{}", minimum_buckets("H..H".into()));
}

#[cfg(test)]
mod tests {
    use super::minimum_buckets;

    #[test]
    fn example_one() {
        assert_eq!(minimum_buckets("H..H".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_buckets(".H.H.".into()), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_buckets(".HHH.".into()), -1);
    }
}
