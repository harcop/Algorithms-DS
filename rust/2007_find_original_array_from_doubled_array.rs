/// LeetCode #2007 - Find Original Array From Doubled Array
use std::collections::HashMap;

fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
    let mut changed = changed;
    changed.sort_unstable();
    let mut cnt = HashMap::new();
    for &x in &changed {
        *cnt.entry(x).or_insert(0) += 1;
    }
    let mut ans = Vec::new();
    for &x in &changed {
        if cnt.get(&x).copied().unwrap_or(0) == 0 {
            continue;
        }
        *cnt.get_mut(&x).unwrap() -= 1;
        if cnt.get(&(x << 1)).copied().unwrap_or(0) <= 0 {
            return Vec::new();
        }
        *cnt.get_mut(&(x << 1)).unwrap() -= 1;
        ans.push(x);
    }
    ans
}

fn main() {
    println!("{:?}", find_original_array(vec![1, 3, 4, 2, 6, 8]));
}

#[cfg(test)]
mod tests {
    use super::find_original_array;

    #[test]
    fn example_one() {
        assert_eq!(
            find_original_array(vec![1, 3, 4, 2, 6, 8]),
            vec![1, 3, 4]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(find_original_array(vec![6, 3, 0, 1]), Vec::<i32>::new());
    }
}
