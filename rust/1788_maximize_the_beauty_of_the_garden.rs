/// LeetCode #1788 - Maximize the Beauty of the Garden
use std::collections::HashMap;

fn maximum_beauty(flowers: Vec<i32>) -> i32 {
    let mut prefix = vec![0i32; flowers.len() + 1];
    let mut first: HashMap<i32, usize> = HashMap::new();
    let mut ans = i32::MIN;

    for (i, &v) in flowers.iter().enumerate() {
        if let Some(&j) = first.get(&v) {
            let beauty = prefix[i] - prefix[j + 1] + v * 2;
            ans = ans.max(beauty);
        } else {
            first.insert(v, i);
        }
        prefix[i + 1] = prefix[i] + v.max(0);
    }

    ans
}

fn main() {
    println!("{}", maximum_beauty(vec![1, 2, 3, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::maximum_beauty;

    #[test]
    fn example_one() {
        assert_eq!(maximum_beauty(vec![1, 2, 3, 1, 2]), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_beauty(vec![100, 1, 1, -3, 1]), 3);
    }

    #[test]
    fn example_three() {
        assert_eq!(maximum_beauty(vec![-1, -2, 0, -1]), -2);
    }
}
