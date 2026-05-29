/// LeetCode #1521 - Find A Value Of A Mysterious Function Closest To Target
fn closest_merging(arr: Vec<i32>, target: i32) -> i32 {
    let mut ans = i32::MAX;
    let mut cur: Vec<i32> = vec![];
    for &x in &arr {
        let mut nxt = vec![x];
        for &v in &cur {
            nxt.push(v & x);
        }
        nxt.sort_unstable();
        nxt.dedup();
        for v in &nxt {
            ans = ans.min((v - target).abs());
        }
        cur = nxt;
    }
    ans
}

fn main() {
    println!("{}", closest_merging(vec![9, 12, 3, 7, 15], 5));
}

#[cfg(test)]
mod tests {
    use super::closest_merging;

    #[test]
    fn example_one() {
        assert_eq!(closest_merging(vec![9, 12, 3, 7, 15], 5), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(closest_merging(vec![1000000, 1000000, 1000000], 1), 999999);
    }
}
