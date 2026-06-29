/// LeetCode #2178 - Maximum Split of Positive Even Integers
fn maximum_even_split(final_sum: i64) -> Vec<i64> {
    if final_sum % 2 == 1 {
        return Vec::new();
    }

    let mut remaining = final_sum;
    let mut next = 2i64;
    let mut ans = Vec::new();
    while remaining >= next {
        ans.push(next);
        remaining -= next;
        next += 2;
    }
    if let Some(last) = ans.last_mut() {
        *last += remaining;
    }
    ans
}

fn main() {
    println!("{:?}", maximum_even_split(12));
}

#[cfg(test)]
mod tests {
    use super::maximum_even_split;

    #[test]
    fn example_one() {
        assert_eq!(maximum_even_split(12), vec![2, 4, 6]);
    }

    #[test]
    fn example_two() {
        assert!(maximum_even_split(7).is_empty());
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_even_split(28), vec![2, 4, 6, 16]);
    }
}
