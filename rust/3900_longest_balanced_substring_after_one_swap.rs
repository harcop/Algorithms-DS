/// LeetCode #3900 - Longest Balanced Substring After One Swap
use std::collections::HashMap;

fn longest_balanced(s: String) -> i32 {
    let s = s.as_bytes();
    let len = s.len();
    let cnt0 = s.iter().filter(|&&c| c == b'0').count();
    let cnt1 = len - cnt0;
    let mut pos: HashMap<i32, Vec<i32>> = HashMap::new();
    pos.insert(0, vec![-1]);
    let mut ans = 0i32;
    let mut pre = 0i32;
    for (i, &c) in s.iter().enumerate() {
        let i = i as i32;
        pre += if c == b'1' { 1 } else { -1 };
        pos.entry(pre).or_default().push(i);
        let first = pos[&pre][0];
        ans = ans.max(i - first);
        if let Some(p) = pos.get(&(pre - 2)) {
            if (i - p[0] - 2) / 2 < cnt0 as i32 {
                ans = ans.max(i - p[0]);
            } else if p.len() > 1 {
                ans = ans.max(i - p[1]);
            }
        }
        if let Some(p) = pos.get(&(pre + 2)) {
            if (i - p[0] - 2) / 2 < cnt1 as i32 {
                ans = ans.max(i - p[0]);
            } else if p.len() > 1 {
                ans = ans.max(i - p[1]);
            }
        }
    }
    ans
}

fn main() {
    println!("{}", longest_balanced("100001".into()));
}

#[cfg(test)]
mod tests {
    use super::longest_balanced;

    #[test]
    fn example1() {
        assert_eq!(longest_balanced("100001".into()), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_balanced("111".into()), 0);
    }
}
