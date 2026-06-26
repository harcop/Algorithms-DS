/// LeetCode #2107 - Number of Unique Flavors After Sharing K Candies
use std::collections::HashMap;

fn share_candies(candies: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut cnt = HashMap::new();
    for &c in &candies {
        *cnt.entry(c).or_insert(0) += 1;
    }

    let mut unique = cnt.len() as i32;
    let mut ans = 0;
    for i in 0..candies.len() {
        let entry = cnt.get_mut(&candies[i]).unwrap();
        *entry -= 1;
        if *entry == 0 {
            unique -= 1;
        }

        if i >= k {
            let entry = cnt.get_mut(&candies[i - k]).unwrap();
            if *entry == 0 {
                unique += 1;
            }
            *entry += 1;
        }

        if i + 1 >= k {
            ans = ans.max(unique);
        }
    }

    ans
}

fn main() {
    println!("{}", share_candies(vec![1, 2, 2, 3, 4, 3], 3));
}

#[cfg(test)]
mod tests {
    use super::share_candies;

    #[test]
    fn keeps_most_unique_flavors() {
        assert_eq!(share_candies(vec![1, 2, 2, 3, 4, 3], 3), 3);
    }

    #[test]
    fn sharing_all_candies_leaves_none() {
        assert_eq!(share_candies(vec![1, 2, 3], 3), 0);
    }

    #[test]
    fn duplicate_window_choice() {
        assert_eq!(share_candies(vec![1, 1, 2, 2, 3, 3], 2), 3);
    }
}
