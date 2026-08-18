/// LeetCode #3267 - Count Almost Equal Pairs II
use std::collections::{HashMap, HashSet};

fn count_pairs(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut ans = 0;
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for x in nums {
        let mut vis = HashSet::new();
        vis.insert(x);
        let mut s: Vec<u8> = x.to_string().into_bytes();
        let m = s.len();
        for j in 0..m {
            for i in 0..j {
                s.swap(i, j);
                vis.insert(std::str::from_utf8(&s).unwrap().parse().unwrap());
                for q in 0..m {
                    for p in 0..q {
                        s.swap(p, q);
                        vis.insert(std::str::from_utf8(&s).unwrap().parse().unwrap());
                        s.swap(p, q);
                    }
                }
                s.swap(i, j);
            }
        }
        for y in vis {
            ans += cnt.get(&y).copied().unwrap_or(0);
        }
        *cnt.entry(x).or_insert(0) += 1;
    }
    ans
}

fn main() {
    println!("{}", count_pairs(vec![1023, 2310, 2130, 213]));
}

#[cfg(test)]
mod tests {
    use super::count_pairs;

    #[test]
    fn example1() {
        assert_eq!(count_pairs(vec![1023, 2310, 2130, 213]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(count_pairs(vec![1, 10, 100]), 3);
    }
}
