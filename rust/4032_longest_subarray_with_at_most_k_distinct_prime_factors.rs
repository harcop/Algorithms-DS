/// LeetCode #4032 - Longest Subarray With at Most K Distinct Prime Factors
fn longest_subarray(nums: Vec<i32>, k: i32) -> i32 {
    const MX: usize = 100_001;
    let mut factors = vec![Vec::new(); MX];
    for i in 2..MX {
        if factors[i].is_empty() {
            let mut j = i;
            while j < MX {
                factors[j].push(i as i32);
                j += i;
            }
        }
    }
    let mut cnt = std::collections::HashMap::<i32, i32>::new();
    let mut ans = 0i32;
    let mut l = 0usize;
    for (r, &x) in nums.iter().enumerate() {
        for &p in &factors[x as usize] {
            *cnt.entry(p).or_insert(0) += 1;
        }
        while cnt.len() > k as usize {
            for &p in &factors[nums[l] as usize] {
                let e = cnt.get_mut(&p).unwrap();
                *e -= 1;
                if *e == 0 {
                    cnt.remove(&p);
                }
            }
            l += 1;
        }
        ans = ans.max((r - l + 1) as i32);
    }
    ans
}

fn main() {
    println!("{}", longest_subarray(vec![7, 6, 10, 12, 11], 3));
}

#[cfg(test)]
mod tests {
    use super::longest_subarray;

    #[test]
    fn example1() {
        assert_eq!(longest_subarray(vec![7, 6, 10, 12, 11], 3), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_subarray(vec![4, 6, 9, 18], 4), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(longest_subarray(vec![6, 10, 15], 2), 1);
    }
}
