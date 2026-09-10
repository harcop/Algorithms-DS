/// LeetCode #3679 - Minimum Discards to Balance Inventory
use std::collections::HashMap;

fn min_arrivals_to_discard(arrivals: Vec<i32>, w: i32, m: i32) -> i32 {
    let w = w as usize;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let n = arrivals.len();
    let mut marked = vec![0i32; n];
    let mut ans = 0;
    for i in 0..n {
        let x = arrivals[i];
        if i >= w {
            let prev = arrivals[i - w];
            *cnt.entry(prev).or_insert(0) -= marked[i - w];
        }
        if *cnt.get(&x).unwrap_or(&0) >= m {
            ans += 1;
        } else {
            marked[i] = 1;
            *cnt.entry(x).or_insert(0) += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", min_arrivals_to_discard(vec![1, 2, 1, 3, 1], 4, 2));
}

#[cfg(test)]
mod tests {
    use super::min_arrivals_to_discard;

    #[test]
    fn example1() {
        assert_eq!(min_arrivals_to_discard(vec![1, 2, 1, 3, 1], 4, 2), 0);
    }

    #[test]
    fn example2() {
        assert_eq!(min_arrivals_to_discard(vec![1, 2, 3, 3, 3, 4], 3, 2), 1);
    }
}
