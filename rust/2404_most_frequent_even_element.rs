/// LeetCode #2404 - Most Frequent Even Element
use std::collections::HashMap;

fn most_frequent_even(nums: Vec<i32>) -> i32 {
    let mut freq = HashMap::new();
    for num in nums {
        if num % 2 == 0 {
            *freq.entry(num).or_insert(0) += 1;
        }
    }

    let mut ans = -1;
    let mut best = 0;
    for (num, count) in freq {
        if count > best || (count == best && (ans == -1 || num < ans)) {
            best = count;
            ans = num;
        }
    }
    ans
}

fn main() {
    println!("{}", most_frequent_even(vec![0, 1, 2, 2, 4, 4, 1]));
}

#[cfg(test)]
mod tests {
    use super::most_frequent_even;

    #[test]
    fn example_one() {
        assert_eq!(most_frequent_even(vec![0, 1, 2, 2, 4, 4, 1]), 2);
    }

    #[test]
    fn no_even_numbers() {
        assert_eq!(most_frequent_even(vec![29, 47, 21, 41, 13, 37, 25, 7]), -1);
    }
}
