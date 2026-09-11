/// LeetCode #3697 - Compute Decimal Representation
fn decimal_representation(mut n: i32) -> Vec<i32> {
    let mut ans = Vec::new();
    let mut p = 1i32;
    while n > 0 {
        let v = n % 10;
        n /= 10;
        if v != 0 {
            ans.push(p * v);
        }
        p *= 10;
    }
    ans.reverse();
    ans
}

fn main() {
    println!("{:?}", decimal_representation(537));
}

#[cfg(test)]
mod tests {
    use super::decimal_representation;

    #[test]
    fn example1() {
        assert_eq!(decimal_representation(537), vec![500, 30, 7]);
    }

    #[test]
    fn example2() {
        assert_eq!(decimal_representation(102), vec![100, 2]);
    }

    #[test]
    fn example3() {
        assert_eq!(decimal_representation(6), vec![6]);
    }
}
