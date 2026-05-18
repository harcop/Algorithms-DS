/// LeetCode #898 - Bitwise ORs of Subarrays
use std::collections::HashSet;

fn subarray_bitwise_ors(arr: Vec<i32>) -> i32 {
    let mut cur: HashSet<i32> = HashSet::new();
    let mut all: HashSet<i32> = HashSet::new();
    for x in arr {
        let mut nxt: HashSet<i32> = HashSet::new();
        nxt.insert(x);
        for &y in &cur {
            nxt.insert(y | x);
        }
        all.extend(nxt.iter().copied());
        cur = nxt;
    }
    all.len() as i32
}

fn main() {
    println!("{}", subarray_bitwise_ors(vec![1, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::subarray_bitwise_ors;

    #[test]
    fn example_one() {
        assert_eq!(subarray_bitwise_ors(vec![0]), 1);
    }
}
