/// LeetCode #2164 - Sort Even and Odd Indices Independently
fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
    let mut even = Vec::new();
    let mut odd = Vec::new();
    for (i, &x) in nums.iter().enumerate() {
        if i % 2 == 0 {
            even.push(x);
        } else {
            odd.push(x);
        }
    }

    even.sort_unstable();
    odd.sort_unstable_by(|a, b| b.cmp(a));

    let mut ans = Vec::with_capacity(nums.len());
    let mut e = 0usize;
    let mut o = 0usize;
    for i in 0..nums.len() {
        if i % 2 == 0 {
            ans.push(even[e]);
            e += 1;
        } else {
            ans.push(odd[o]);
            o += 1;
        }
    }
    ans
}

fn main() {
    println!("{:?}", sort_even_odd(vec![4, 1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::sort_even_odd;

    #[test]
    fn example_one() {
        assert_eq!(sort_even_odd(vec![4, 1, 2, 3]), vec![2, 3, 4, 1]);
    }

    #[test]
    fn example_two() {
        assert_eq!(sort_even_odd(vec![2, 1]), vec![2, 1]);
    }
}
