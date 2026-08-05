/// LeetCode #3020 - Find the Maximum Number of Elements in Subset
use std::collections::HashMap;

fn maximum_length(nums: Vec<i32>) -> i32 {
    let mut cnt: HashMap<i64, i32> = HashMap::new();
    for x in nums {
        *cnt.entry(x as i64).or_insert(0) += 1;
    }

    let c1 = *cnt.get(&1).unwrap_or(&0);
    let mut ans = if c1 == 0 {
        0
    } else {
        c1 - (1 - c1 % 2)
    };
    cnt.remove(&1);

    for &x in cnt.keys().collect::<Vec<_>>().iter() {
        let mut t = 0;
        let mut cur = *x;
        while *cnt.get(&cur).unwrap_or(&0) > 1 {
            cur = cur * cur;
            t += 2;
        }
        if *cnt.get(&cur).unwrap_or(&0) > 0 {
            t += 1;
        } else {
            t -= 1;
        }
        ans = ans.max(t);
    }

    ans
}

fn main() {
    println!("{}", maximum_length(vec![5, 4, 1, 2, 2]));
    println!("{}", maximum_length(vec![1, 3, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::maximum_length;

    #[test]
    fn example_one() {
        assert_eq!(maximum_length(vec![5, 4, 1, 2, 2]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_length(vec![1, 3, 2, 4]), 1);
    }
}
