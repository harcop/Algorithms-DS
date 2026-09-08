/// LeetCode #3645 - Maximum Total from Optimal Activation Order
use std::collections::HashMap;

fn max_total(value: Vec<i32>, limit: Vec<i32>) -> i64 {
    let mut g: HashMap<i32, Vec<i32>> = HashMap::new();
    for (v, lim) in value.into_iter().zip(limit) {
        g.entry(lim).or_default().push(v);
    }
    let mut ans = 0i64;
    for (lim, mut vs) in g {
        vs.sort_unstable_by(|a, b| b.cmp(a));
        let take = lim.min(vs.len() as i32) as usize;
        for i in 0..take {
            ans += vs[i] as i64;
        }
    }
    ans
}

fn main() {
    println!("{}", max_total(vec![3, 5, 8], vec![2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::max_total;

    #[test]
    fn example1() {
        assert_eq!(max_total(vec![3, 5, 8], vec![2, 1, 3]), 16);
    }

    #[test]
    fn example2() {
        assert_eq!(max_total(vec![4, 2, 6], vec![1, 1, 1]), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(max_total(vec![4, 1, 5, 2], vec![3, 3, 2, 3]), 12);
    }
}
