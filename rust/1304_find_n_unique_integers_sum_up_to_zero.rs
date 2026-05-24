/// LeetCode #1304 - Find N Unique Integers Sum up to Zero
fn sum_zero(n: i32) -> Vec<i32> {
    let mut ans = vec![];
    for i in 1..=n / 2 {
        ans.push(i);
        ans.push(-i);
    }
    if n % 2 == 1 {
        ans.push(0);
    }
    ans
}

fn main() {
    println!("{:?}", sum_zero(5));
}

#[cfg(test)]
mod tests {
    use super::sum_zero;

    #[test]
    fn example_one() {
        let v = sum_zero(5);
        assert_eq!(v.len(), 5);
        assert_eq!(v.iter().sum::<i32>(), 0);
    }

    #[test]
    fn example_two() {
        let v = sum_zero(3);
        assert_eq!(v.len(), 3);
        assert_eq!(v.iter().sum::<i32>(), 0);
    }
}
