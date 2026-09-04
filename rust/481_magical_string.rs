/// LeetCode #481 - Magical String
fn magical_string(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    if n <= 3 {
        return 1;
    }
    let n = n as usize;
    let mut s = vec![1, 2, 2];
    let mut i = 2;
    while s.len() < n {
        let next = 3 - s[s.len() - 1];
        for _ in 0..s[i] {
            s.push(next);
        }
        i += 1;
    }
    s[..n].iter().filter(|&&x| x == 1).count() as i32
}

fn main() {
    println!("{}", magical_string(6));
}

#[cfg(test)]
mod tests {
    use super::magical_string;

    #[test]
    fn example_one() {
        assert_eq!(magical_string(6), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(magical_string(1), 1);
    }
}
