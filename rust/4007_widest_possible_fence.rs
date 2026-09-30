/// LeetCode #4007 - Widest Possible Fence
use std::collections::HashMap;

fn maximum_width(planks: Vec<i32>) -> i32 {
    let mut cnt: HashMap<i64, i32> = HashMap::new();
    for x in planks {
        *cnt.entry(x as i64).or_insert(0) += 1;
    }
    let keys: Vec<(i64, i32)> = cnt.iter().map(|(&k, &v)| (k, v)).collect();
    let mut t: HashMap<i64, i32> = HashMap::new();
    let mut ans = 0i32;
    for &(x, v1) in &keys {
        let e = t.entry(x).or_insert(0);
        *e += v1;
        ans = ans.max(*e);
        let e = t.entry(x * 2).or_insert(0);
        *e += v1 / 2;
        ans = ans.max(*e);
        for &(y, v2) in &keys {
            if y > x {
                let e = t.entry(x + y).or_insert(0);
                *e += v1.min(v2);
                ans = ans.max(*e);
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        maximum_width(vec![1, 3, 2, 5, 7, 5, 4, 2, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_width;

    #[test]
    fn example1() {
        assert_eq!(maximum_width(vec![1, 3, 2, 5, 7, 5, 4, 2, 1]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_width(vec![2, 3, 7]), 1);
    }
}
