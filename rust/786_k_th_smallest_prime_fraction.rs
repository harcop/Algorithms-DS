/// LeetCode #786 - K-th Smallest Prime Fraction
fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
    let mut fracs: Vec<(i32, i32)> = vec![];
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            fracs.push((arr[i], arr[j]));
        }
    }
    fracs.sort_by(|a, b| (a.0 as i64 * b.1 as i64).cmp(&(b.0 as i64 * a.1 as i64)));
    let (x, y) = fracs[k as usize - 1];
    vec![x, y]
}

fn main() {
    println!("{:?}", kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3));
}

#[cfg(test)]
mod tests {
    use super::kth_smallest_prime_fraction;

    #[test]
    fn example_one() {
        assert_eq!(kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3), vec![2, 5]);
    }
}
