/// LeetCode #1925 - Count Square Sum Triples
fn count_triples(n: i32) -> i32 {
    let n = n as i64;
    let mut ans = 0i32;
    for a in 1..n {
        for b in 1..n {
            let x = a * a + b * b;
            let c = (x as f64).sqrt() as i64;
            if c <= n && c * c == x {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", count_triples(5));
}

#[cfg(test)]
mod tests {
    use super::count_triples;

    #[test]
    fn example_one() {
        assert_eq!(count_triples(5), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_triples(10), 4);
    }
}
