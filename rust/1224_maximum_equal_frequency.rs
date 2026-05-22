/// LeetCode #1224 - Maximum Equal Frequency
fn max_equal_freq(nums: Vec<i32>) -> i32 {
    let mut cnt = std::collections::HashMap::new();
    let mut ccnt = std::collections::HashMap::new();
    let mut ans = 0i32;
    let mut mx = 0i32;
    for (i, v) in nums.iter().enumerate() {
        let i = (i + 1) as i32;
        if let Some(&c) = cnt.get(v) {
            *ccnt.get_mut(&c).unwrap() -= 1;
            if ccnt[&c] == 0 {
                ccnt.remove(&c);
            }
        }
        let c = cnt.entry(*v).or_insert(0);
        *c += 1;
        mx = mx.max(*c);
        *ccnt.entry(*c).or_insert(0) += 1;
        if mx == 1 {
            ans = i;
        } else if ccnt.get(&mx).copied().unwrap_or(0) * mx
            + ccnt.get(&(mx - 1)).copied().unwrap_or(0) * (mx - 1)
            == i
            && ccnt.get(&mx).copied().unwrap_or(0) == 1
        {
            ans = i;
        } else if ccnt.get(&mx).copied().unwrap_or(0) * mx + 1 == i
            && ccnt.get(&1).copied().unwrap_or(0) == 1
        {
            ans = i;
        }
    }
    ans
}

fn main() {
    println!("{}", max_equal_freq(vec![2, 2, 1, 1, 5, 3, 3, 5]));
}

#[cfg(test)]
mod tests {
    use super::max_equal_freq;

    #[test]
    fn example_one() {
        assert_eq!(max_equal_freq(vec![2, 2, 1, 1, 5, 3, 3, 5]), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_equal_freq(vec![10, 2, 8, 9, 3, 8, 1, 5, 2, 3, 7, 6]), 8);
    }
}
