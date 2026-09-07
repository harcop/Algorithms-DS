/// LeetCode #3618 - Split Array by Prime Indices
fn split_array(nums: Vec<i32>) -> i64 {
    let m = nums.len().max(10);
    let mut primes = vec![true; m];
    if m > 0 {
        primes[0] = false;
    }
    if m > 1 {
        primes[1] = false;
    }
    let mut i = 2;
    while i * i < m {
        if primes[i] {
            let mut j = i * i;
            while j < m {
                primes[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    let mut ans = 0i64;
    for (i, x) in nums.into_iter().enumerate() {
        if primes[i] {
            ans += x as i64;
        } else {
            ans -= x as i64;
        }
    }
    ans.abs()
}

fn main() {
    println!("{}", split_array(vec![2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::split_array;

    #[test]
    fn example1() {
        assert_eq!(split_array(vec![2, 3, 4]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(split_array(vec![-1, 5, 7, 0]), 3);
    }
}
