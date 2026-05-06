/// LeetCode #393 - UTF-8 Validation
fn valid_utf8(data: Vec<i32>) -> bool {
    let mut need = 0i32;
    for x in data {
        let b = (x & 0xFF) as i32;
        if need == 0 {
            if (b >> 7) == 0 {
                continue;
            } else if (b >> 5) == 0b110 {
                need = 1;
            } else if (b >> 4) == 0b1110 {
                need = 2;
            } else if (b >> 3) == 0b11110 {
                need = 3;
            } else {
                return false;
            }
        } else {
            if (b >> 6) != 0b10 {
                return false;
            }
            need -= 1;
        }
    }
    need == 0
}

fn main() {
    println!("{}", valid_utf8(vec![197, 130, 1]));
}

#[cfg(test)]
mod tests {
    use super::valid_utf8;

    #[test]
    fn example_one() {
        assert!(valid_utf8(vec![197, 130, 1]));
    }

    #[test]
    fn example_two() {
        assert!(!valid_utf8(vec![235, 140, 4]));
    }
}
