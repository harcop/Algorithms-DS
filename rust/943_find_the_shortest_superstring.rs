/// LeetCode #943 - Find the Shortest Superstring

fn shortest_superstring(words: Vec<String>) -> String {
    let n = words.len();
    let mut overlap = vec![vec![0i32; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let a = words[i].as_bytes();
            let b = words[j].as_bytes();
            let max_k = a.len().min(b.len());
            for k in (1..=max_k).rev() {
                if a[a.len() - k..] == b[..k] {
                    overlap[i][j] = k as i32;
                    break;
                }
            }
        }
    }
    let mut dp = vec![vec![0i32; n]; 1 << n];
    let mut parent = vec![vec![-1i32; n]; 1 << n];
    for mask in 1..(1 << n) {
        for j in 0..n {
            if mask & (1 << j) == 0 {
                continue;
            }
            let prev_mask = mask ^ (1 << j);
            if prev_mask == 0 {
                continue;
            }
            for i in 0..n {
                if prev_mask & (1 << i) == 0 {
                    continue;
                }
                let cand = dp[prev_mask as usize][i] + overlap[i][j];
                if cand > dp[mask as usize][j] {
                    dp[mask as usize][j] = cand;
                    parent[mask as usize][j] = i as i32;
                }
            }
        }
    }
    let full = (1 << n) - 1;
    let mut end = 0usize;
    for j in 0..n {
        if dp[full as usize][j] > dp[full as usize][end] {
            end = j;
        }
    }
    let mut order = Vec::new();
    let mut mask = full;
    let mut cur = end as i32;
    while cur >= 0 {
        let c = cur as usize;
        order.push(c);
        let p = parent[mask as usize][c];
        mask ^= 1 << c;
        cur = p;
    }
    order.reverse();
    let mut ans = words[order[0]].clone();
    for k in 1..order.len() {
        let i = order[k - 1];
        let j = order[k];
        let ov = overlap[i][j] as usize;
        ans.push_str(&words[j][ov..]);
    }
    ans
}

fn main() {
    println!("{}", shortest_superstring(vec!["alex".into(), "loves".into(), "leetcode".into()]));
}

#[cfg(test)]
mod tests {
    use super::shortest_superstring;

    #[test]
    fn example_one() {
        let s = shortest_superstring(vec!["alex".into(), "loves".into(), "leetcode".into()]);
        assert!(s.len() <= 17);
        for w in ["alex", "loves", "leetcode"] {
            assert!(s.contains(w));
        }
    }
}
