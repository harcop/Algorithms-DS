/// LeetCode #3032 - Count Numbers with Unique Digits II
fn has_unique_digits(x: i32) -> bool {
    let mut seen = [false; 10];
    let mut n = x;
    while n > 0 {
        let d = (n % 10) as usize;
        if seen[d] {
            return false;
        }
        seen[d] = true;
        n /= 10;
    }
    true
}

fn count_unique(a: i32, b: i32) -> i32 {
    (a..=b).filter(|&x| has_unique_digits(x)).count() as i32
}

fn main() {
    println!("{}", count_unique(1, 20));
}

#[cfg(test)]
mod tests {
    use super::count_unique;

    #[test]
    fn example1() {
        assert_eq!(count_unique(1, 20), 19);
    }

    #[test]
    fn example2() {
        assert_eq!(count_unique(9, 19), 10);
    }

    #[test]
    fn example3() {
        assert_eq!(count_unique(80, 120), 27);
    }
}
