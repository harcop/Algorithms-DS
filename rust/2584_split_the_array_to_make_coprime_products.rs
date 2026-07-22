/// LeetCode #2584 - Split the Array to Make Coprime Products
use std::collections::HashMap;

fn find_valid_split(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut first: HashMap<i32, usize> = HashMap::new();
    let mut last: Vec<usize> = (0..n).collect();

    for (i, &num) in nums.iter().enumerate() {
        let mut x = num;
        let mut j = 2;
        while j <= x / j {
            if x % j == 0 {
                if let Some(&k) = first.get(&j) {
                    last[k] = i;
                } else {
                    first.insert(j, i);
                }
                while x % j == 0 {
                    x /= j;
                }
            }
            j += 1;
        }
        if x > 1 {
            if let Some(&k) = first.get(&x) {
                last[k] = i;
            } else {
                first.insert(x, i);
            }
        }
    }

    let mut mx = last[0];
    for (i, &x) in last.iter().enumerate() {
        if mx < i {
            return mx as i32;
        }
        mx = mx.max(x);
    }
    -1
}

fn main() {
    println!("{}", find_valid_split(vec![4, 7, 8, 15, 3, 5]));
}

#[cfg(test)]
mod tests {
    use super::find_valid_split;

    #[test]
    fn example_one() {
        assert_eq!(find_valid_split(vec![4, 7, 8, 15, 3, 5]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_valid_split(vec![4, 7, 15, 8, 3, 5]), -1);
    }
}
