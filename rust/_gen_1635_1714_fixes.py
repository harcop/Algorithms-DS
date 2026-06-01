"""Overrides for solutions needing correction."""

FIXES: dict[int, str] = {
1637: r'''fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    let mut xs: Vec<i32> = points.iter().map(|p| p[0]).collect();
    xs.sort_unstable();
    xs.dedup();
    let mut ans = 0i32;
    for w in xs.windows(2) { ans = ans.max(w[1] - w[0]); }
    ans
}
fn main() { println!("{}", max_width_of_vertical_area(vec![vec![8,7],vec![9,9],vec![7,4],vec![9,7]])); }
#[cfg(test)]
mod tests {
    use super::max_width_of_vertical_area;
    #[test]
    fn example_one() { assert_eq!(max_width_of_vertical_area(vec![vec![8,7],vec![9,9],vec![7,4],vec![9,7]]), 1); }
    #[test]
    fn example_two() { assert_eq!(max_width_of_vertical_area(vec![vec![3,1],vec![9,0],vec![1,0],vec![1,4],vec![5,3],vec![8,8]]), 3); }
}''',

1638: r'''fn count_substrings(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut ans = 0i32;
    for i in 0..s.len() {
        for j in 0..t.len() {
            let mut diff = 0i32;
            for k in 0..s.len().min(t.len()).min(s.len() - i).min(t.len() - j) {
                if s[i + k] != t[j + k] {
                    diff += 1;
                    if diff > 1 { break; }
                }
                if diff == 1 { ans += 1; }
            }
        }
    }
    ans
}
fn main() { println!("{}", count_substrings("aba".into(), "baba".into())); }
#[cfg(test)]
mod tests {
    use super::count_substrings;
    #[test]
    fn example_one() { assert_eq!(count_substrings("aba".into(), "baba".into()), 6); }
    #[test]
    fn example_two() { assert_eq!(count_substrings("ab".into(), "bb".into()), 3); }
}''',

1639: r'''const MOD: i64 = 1_000_000_007;

fn num_ways(words: Vec<String>, target: String) -> i32 {
    let wlen = words[0].len();
    let mut cnt = vec![vec![0i64; 26]; wlen];
    for w in &words {
        for (j, &c) in w.as_bytes().iter().enumerate() {
            cnt[j][(c - b'a') as usize] += 1;
        }
    }
    let t = target.as_bytes();
    let m = t.len();
    let mut dp = vec![vec![0i64; wlen + 1]; m + 1];
    for j in 0..=wlen { dp[m][j] = 1; }
    for i in (0..m).rev() {
        for j in (0..wlen).rev() {
            dp[i][j] = dp[i][j + 1];
            dp[i][j] = (dp[i][j] + dp[i + 1][j + 1] * cnt[j][(t[i] - b'a') as usize]) % MOD;
        }
    }
    dp[0][0] as i32
}
fn main() { println!("{}", num_ways(vec!["acca".into(),"bbbb".into(),"caca".into()], "aba".into())); }
#[cfg(test)]
mod tests {
    use super::num_ways;
    #[test]
    fn example_one() { assert_eq!(num_ways(vec!["acca".into(),"bbbb".into(),"caca".into()], "aba".into()), 6); }
}''',

1643: r'''fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
    let (mut r, mut c) = (destination[0], destination[1]);
    let mut k = k as i64;
    let mut ans = String::new();
    fn comb(n: i64, k: i64) -> i64 {
        if k < 0 || k > n { return 0; }
        if k == 0 || k == n { return 1; }
        let k = k.min(n - k);
        let mut num = 1i64;
        let mut den = 1i64;
        for i in 0..k {
            num *= n - i;
            den *= i + 1;
        }
        num / den
    }
    while r > 0 || c > 0 {
        let right = comb((r + c - 1) as i64, (c - 1) as i64);
        if k <= right {
            ans.push('H');
            c -= 1;
        } else {
            ans.push('V');
            k -= right;
            r -= 1;
        }
    }
    ans
}
fn main() { println!("{}", kth_smallest_path(vec![2,3], 1)); }
#[cfg(test)]
mod tests {
    use super::kth_smallest_path;
    #[test]
    fn example_one() { assert_eq!(kth_smallest_path(vec![2,3], 1), "HHHVV"); }
    #[test]
    fn example_two() { assert_eq!(kth_smallest_path(vec![2,3], 2), "HHVHV"); }
}''',

1644: r'''#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn lowest_common_ancestor(root: &Option<Box<TreeNode>>, p: &Option<Box<TreeNode>>, q: &Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    let p = p.as_ref()?.val;
    let q = q.as_ref()?.val;
    fn dfs(r: &Option<Box<TreeNode>>, p: i32, q: i32) -> (bool, bool, Option<i32>) {
        let Some(n) = r else { return (false, false, None); };
        let (lp, lq, ll) = dfs(&n.left, p, q);
        let (rp, rq, rl) = dfs(&n.right, p, q);
        let has_p = lp || rp || n.val == p;
        let has_q = lq || rq || n.val == q;
        if ll.is_some() { return (has_p, has_q, ll); }
        if rl.is_some() { return (has_p, has_q, rl); }
        if has_p && has_q { return (true, true, Some(n.val)); }
        (has_p, has_q, None)
    }
    let val = dfs(root, p, q).2?;
    Some(Box::new(TreeNode { val, left: None, right: None }))
}
fn main() { println!("{:?}", lowest_common_ancestor(&None, &None, &None)); }
#[cfg(test)]
mod tests {
    use super::{lowest_common_ancestor, TreeNode};
    fn build(vals: Vec<Option<i32>>) -> Option<Box<TreeNode>> {
        if vals.is_empty() || vals[0].is_none() { return None; }
        let mut nodes: Vec<Option<Box<TreeNode>>> = vals.iter().map(|&v| {
            v.map(|x| Box::new(TreeNode { val: x, left: None, right: None }))
        }).collect();
        for i in 0..nodes.len() {
            if nodes[i].is_none() { continue; }
            let l = 2 * i + 1;
            let r = 2 * i + 2;
            if l < nodes.len() { nodes[i].as_mut().unwrap().left = nodes[l].take(); }
            if r < nodes.len() { nodes[i].as_mut().unwrap().right = nodes[r].take(); }
        }
        nodes.remove(0)
    }
    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 5,
                left: Some(Box::new(TreeNode { val: 6, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 2, left: None, right: None })),
            })),
            right: Some(Box::new(TreeNode {
                val: 1,
                left: Some(Box::new(TreeNode { val: 0, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 8, left: None, right: None })),
            })),
        }));
        let p = Some(Box::new(TreeNode { val: 5, left: None, right: None }));
        let q = Some(Box::new(TreeNode { val: 1, left: None, right: None }));
        assert_eq!(lowest_common_ancestor(&root, &p, &q).unwrap().val, 3);
    }
}''',

1647: r'''use std::collections::HashSet;

fn min_deletions(s: String) -> i32 {
    let mut cnt = [0i32; 26];
    for c in s.bytes() { cnt[(c - b'a') as usize] += 1; }
    let mut freqs: Vec<i32> = cnt.iter().copied().filter(|&x| x > 0).collect();
    freqs.sort_unstable_by(|a, b| b.cmp(a));
    let mut seen = HashSet::new();
    let mut del = 0i32;
    for f in freqs {
        let mut x = f;
        while x > 0 && seen.contains(&x) {
            x -= 1;
            del += 1;
        }
        seen.insert(x);
    }
    del
}
fn main() { println!("{}", min_deletions("aaabbbcc".into())); }
#[cfg(test)]
mod tests {
    use super::min_deletions;
    #[test]
    fn example_one() { assert_eq!(min_deletions("aaabbbcc".into()), 2); }
}''',

1650: r'''use std::collections::HashMap;

pub struct Node {
    pub val: i32,
    pub parent: Option<i32>,
}

fn lowest_common_ancestor(p: i32, q: i32, parent: &HashMap<i32, i32>) -> i32 {
    let mut seen = std::collections::HashSet::new();
    let mut cur = p;
    loop {
        seen.insert(cur);
        cur = match parent.get(&cur) {
            Some(&par) => par,
            None => break,
        };
    }
    let mut cur = q;
    loop {
        if seen.contains(&cur) { return cur; }
        cur = match parent.get(&cur) {
            Some(&par) => par,
            None => break,
        };
    }
    -1
}
fn main() {
    let mut parent = HashMap::new();
    parent.insert(5, 3); parent.insert(3, 1); parent.insert(1, 0);
    println!("{}", lowest_common_ancestor(5, 1, &parent));
}
#[cfg(test)]
mod tests {
    use super::lowest_common_ancestor;
    use std::collections::HashMap;
    #[test]
    fn example_one() {
        let mut parent = HashMap::new();
        parent.insert(5, 3); parent.insert(3, 1); parent.insert(1, 0);
        assert_eq!(lowest_common_ancestor(5, 1, &parent), 1);
    }
}''',

1652: r'''fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let n = code.len();
    if k == 0 { return vec![0; n]; }
    let kabs = k.unsigned_abs() as usize;
    let mut ans = vec![0; n];
    for i in 0..n {
        let mut s = 0i32;
        for j in 1..=kabs {
            let idx = if k > 0 { (i + j) % n } else { (i + n - j) % n };
            s += code[idx];
        }
        ans[i] = s;
    }
    ans
}
fn main() { println!("{:?}", decrypt(vec![5,7,1,4], 3)); }
#[cfg(test)]
mod tests {
    use super::decrypt;
    #[test]
    fn example_one() { assert_eq!(decrypt(vec![5,7,1,4], 3), vec![12,10,16,13]); }
    #[test]
    fn example_two() { assert_eq!(decrypt(vec![1,2,3,4], 0), vec![0,0,0,0]); }
}''',

1653: r'''fn minimum_deletions(s: String) -> i32 {
    let mut b = 0i32;
    let mut ans = 0i32;
    for c in s.bytes() {
        if c == b'b' { b += 1; }
        else if b > 0 { ans += 1; b -= 1; }
    }
    ans
}
fn main() { println!("{}", minimum_deletions("aababbab".into())); }
#[cfg(test)]
mod tests {
    use super::minimum_deletions;
    #[test]
    fn example_one() { assert_eq!(minimum_deletions("aababbab".into()), 2); }
}''',

1655: r'''fn can_distribute(nums: Vec<i32>, quantity: Vec<i32>) -> bool {
    let mut cnt = [0i32; 101];
    for x in nums { cnt[x as usize] += 1; }
    let mut qty = quantity;
    qty.sort_unstable_by(|a, b| b.cmp(a));
    fn dfs(cnt: &mut [i32; 101], qty: &[i32], i: usize) -> bool {
        if i == qty.len() { return true; }
        let need = qty[i];
        let mut seen = std::collections::HashSet::new();
        for v in 1..=100 {
            if cnt[v] >= need && seen.insert(cnt[v]) {
                cnt[v] -= need;
                if dfs(cnt, qty, i + 1) { return true; }
                cnt[v] += need;
            }
        }
        false
    }
    dfs(&mut cnt, &qty, 0)
}
fn main() { println!("{}", can_distribute(vec![1,2,3,4], vec![2])); }
#[cfg(test)]
mod tests {
    use super::can_distribute;
    #[test]
    fn example_one() { assert!(can_distribute(vec![1,2,3,4], vec![2])); }
    #[test]
    fn example_two() { assert!(!can_distribute(vec![1,1,2,2], vec![2,2])); }
}''',

1657: r'''fn close_strings(word1: String, word2: String) -> bool {
    let mut c1 = [0i32; 26];
    let mut c2 = [0i32; 26];
    for c in word1.bytes() { c1[(c - b'a') as usize] += 1; }
    for c in word2.bytes() { c2[(c - b'a') as usize] += 1; }
    let mut f1: Vec<i32> = c1.iter().copied().filter(|&x| x > 0).collect();
    let mut f2: Vec<i32> = c2.iter().copied().filter(|&x| x > 0).collect();
    f1.sort_unstable();
    f2.sort_unstable();
    f1 == f2 && c1.iter().zip(c2.iter()).all(|(&a, &b)| (a > 0) == (b > 0))
}
fn main() { println!("{}", close_strings("abc".into(), "bca".into())); }
#[cfg(test)]
mod tests {
    use super::close_strings;
    #[test]
    fn example_one() { assert!(close_strings("abc".into(), "bca".into())); }
    #[test]
    fn example_two() { assert!(!close_strings("a".into(), "aa".into())); }
}''',

1659: r'''fn row_score(row: usize, mask: i32, prev: i32, imask: i32, seats: &[Vec<i32>]) -> i32 {
    let n = seats[0].len();
    let mut g = 0i32;
    for j in 0..n {
        if (mask >> j) & 1 == 0 { continue; }
        if seats[row][j] == 0 { return i32::MIN / 4; }
        let intro = (imask >> j) & 1 == 1;
        g += if intro { 120 } else { 40 };
        if j > 0 && (mask >> (j - 1)) & 1 == 1 { g += if intro { -30 } else { 20 }; }
        if j + 1 < n && (mask >> (j + 1)) & 1 == 1 { g += if intro { -30 } else { 20 }; }
        if j > 0 && (prev >> (j - 1)) & 1 == 1 { g += if intro { -30 } else { 20 }; }
        if j + 1 < n && (prev >> (j + 1)) & 1 == 1 { g += if intro { -30 } else { 20 }; }
    }
    g
}

fn get_max_happy(students: Vec<i32>, seats: Vec<Vec<i32>>) -> i32 {
    let m = seats.len();
    let n = seats[0].len();
    let intro = students[0] as usize;
    let extro = students[1] as usize;
    let mut dp = vec![vec![vec![i32::MIN / 4; extro + 1]; intro + 1]; 1 << n];
    dp[0][0][0] = 0;
    for row in 0..m {
        let mut nd = vec![vec![vec![i32::MIN / 4; extro + 1]; intro + 1]; 1 << n];
        for prev in 0usize..(1 << n) {
            for i in 0..=intro {
                for e in 0..=extro {
                    if dp[prev][i][e] <= i32::MIN / 8 { continue; }
                    for mask in 0usize..(1 << n) {
                        for imask in 0usize..(1 << n) {
                            if imask & mask != imask { continue; }
                            let ic = imask.count_ones() as usize;
                            let ec = mask.count_ones() as usize - ic;
                            if i + ic > intro || e + ec > extro { continue; }
                            let gain = row_score(row, mask as i32, prev as i32, imask as i32, &seats);
                            if gain <= i32::MIN / 8 { continue; }
                            nd[mask][i + ic][e + ec] = nd[mask][i + ic][e + ec].max(dp[prev][i][e] + gain);
                        }
                    }
                }
            }
        }
        dp = nd;
    }
    (0usize..(1 << n)).map(|mask| dp[mask][intro][extro]).max().unwrap_or(0)
}
fn main() { println!("{}", get_max_happy(vec![1,1], vec![vec![1,1],vec![1,1]])); }
#[cfg(test)]
mod tests {
    use super::get_max_happy;
    #[test]
    fn example_one() { assert_eq!(get_max_happy(vec![1,1], vec![vec![1,1],vec![1,1]]), 4); }
}''',

1664: r'''fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut o = 0i64;
    let mut e = 0i64;
    for (i, &x) in nums.iter().enumerate() {
        if i % 2 == 0 { e += x as i64; } else { o += x as i64; }
    }
    let mut po = 0i64;
    let mut pe = 0i64;
    let mut ans = 0i32;
    for i in 0..n {
        let so = o - po - if i % 2 == 1 { nums[i] as i64 } else { 0 };
        let se = e - pe - if i % 2 == 0 { nums[i] as i64 } else { 0 };
        if i % 2 == 0 {
            if pe + so == po + se { ans += 1; }
        } else if po + se == pe + so {
            ans += 1;
        }
        if i % 2 == 0 { pe += nums[i] as i64; } else { po += nums[i] as i64; }
    }
    ans
}
fn main() { println!("{}", ways_to_make_fair(vec![2,1,6,4])); }
#[cfg(test)]
mod tests {
    use super::ways_to_make_fair;
    #[test]
    fn example_one() { assert_eq!(ways_to_make_fair(vec![2,1,6,4]), 1); }
}''',

1666: r'''#[derive(Clone)]
pub struct Node {
    pub val: i32,
    pub children: Vec<Node>,
}

fn flip_binary_tree(root: Option<Box<Node>>, leaf: i32) -> Option<Box<Node>> {
    fn find_path(node: &Node, leaf: i32, path: &mut Vec<i32>) -> bool {
        path.push(node.val);
        if node.val == leaf { return true; }
        for c in &node.children {
            if find_path(c, leaf, path) { return true; }
        }
        path.pop();
        false
    }
    let root = root?;
    let mut path = vec![];
    if !find_path(root.as_ref(), leaf, &mut path) { return Some(root); }
    let mut cur = root;
    for idx in (1..path.len()).rev() {
        let child_val = path[idx];
        let parent_val = path[idx - 1];
        fn detach(node: &mut Node, child_val: i32, parent_val: i32) {
            if node.val == parent_val {
                node.children.retain(|c| c.val != child_val);
                return;
            }
            for c in &mut node.children { detach(c, child_val, parent_val); }
        }
        detach(cur.as_mut(), child_val, parent_val);
        fn extract(node: &mut Node, val: i32) -> Option<Box<Node>> {
            for i in 0..node.children.len() {
                if node.children[i].val == val {
                    let child = node.children.remove(i);
                    return Some(Box::new(child));
                }
            }
            for c in &mut node.children {
                if let Some(x) = extract(c, val) { return Some(x); }
            }
            None
        }
        if let Some(mut nr) = extract(cur.as_mut(), child_val) {
            nr.children.push(*cur);
            cur = nr;
        }
    }
    Some(cur)
}
fn main() { let _ = flip_binary_tree(None, 0); }
#[cfg(test)]
mod tests {
    use super::{flip_binary_tree, Node};
    #[test]
    fn example_one() {
        let root = Some(Box::new(Node {
            val: 3,
            children: vec![
                Node { val: 5, children: vec![Node { val: 6, children: vec![] }] },
                Node { val: 1, children: vec![Node { val: 2, children: vec![Node { val: 7, children: vec![] }] }] },
            ],
        }));
        assert!(flip_binary_tree(root, 7).is_some());
    }
}''',

1668: r'''fn trailing(x: i32) -> (i32, i32) {
    let mut a = 0i32;
    let mut b = 0i32;
    let mut v = x;
    while v % 2 == 0 { a += 1; v /= 2; }
    v = x;
    while v % 5 == 0 { b += 1; v /= 5; }
    (a, b)
}

fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut tl2 = vec![vec![0i32; n]; m];
    let mut tl5 = tl2.clone();
    let mut tr2 = tl2.clone();
    let mut tr5 = tl2.clone();
    let mut bl2 = tl2.clone();
    let mut bl5 = tl2.clone();
    let mut br2 = tl2.clone();
    let mut br5 = tl2.clone();
    for i in 0..m {
        for j in 0..n {
            let (a, b) = trailing(grid[i][j]);
            tl2[i][j] = a + if i > 0 { tl2[i-1][j] } else { 0 } + if j > 0 { tl2[i][j-1] } else { 0 }
                - if i > 0 && j > 0 { tl2[i-1][j-1] } else { 0 };
            tl5[i][j] = b + if i > 0 { tl5[i-1][j] } else { 0 } + if j > 0 { tl5[i][j-1] } else { 0 }
                - if i > 0 && j > 0 { tl5[i-1][j-1] } else { 0 };
        }
    }
    for i in (0..m).rev() {
        for j in 0..n {
            let (a, b) = trailing(grid[i][j]);
            bl2[i][j] = a + if i+1<m { bl2[i+1][j] } else {0} + if j>0 { bl2[i][j-1] } else {0}
                - if i+1<m && j>0 { bl2[i+1][j-1] } else {0};
            bl5[i][j] = b + if i+1<m { bl5[i+1][j] } else {0} + if j>0 { bl5[i][j-1] } else {0}
                - if i+1<m && j>0 { bl5[i+1][j-1] } else {0};
        }
    }
    for i in 0..m {
        for j in (0..n).rev() {
            let (a, b) = trailing(grid[i][j]);
            tr2[i][j] = a + if i>0 { tr2[i-1][j] } else {0} + if j+1<n { tr2[i][j+1] } else {0}
                - if i>0 && j+1<n { tr2[i-1][j+1] } else {0};
            tr5[i][j] = b + if i>0 { tr5[i-1][j] } else {0} + if j+1<n { tr5[i][j+1] } else {0}
                - if i>0 && j+1<n { tr5[i-1][j+1] } else {0};
        }
    }
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            let (a, b) = trailing(grid[i][j]);
            br2[i][j] = a + if i+1<m { br2[i+1][j] } else {0} + if j+1<n { br2[i][j+1] } else {0}
                - if i+1<m && j+1<n { br2[i+1][j+1] } else {0};
            br5[i][j] = b + if i+1<m { br5[i+1][j] } else {0} + if j+1<n { br5[i][j+1] } else {0}
                - if i+1<m && j+1<n { br5[i+1][j+1] } else {0};
        }
    }
    let mut ans = 0i32;
    for i in 0..m {
        for j in 0..n {
            let (c2, c5) = trailing(grid[i][j]);
            for (t2, t5) in [
                (tl2[i][j] + br2[i][j] - c2, tl5[i][j] + br5[i][j] - c5),
                (tl2[i][j] + bl2[i][j] - c2, tl5[i][j] + bl5[i][j] - c5),
                (tr2[i][j] + bl2[i][j] - c2, tr5[i][j] + bl5[i][j] - c5),
                (tr2[i][j] + br2[i][j] - c2, tr5[i][j] + br5[i][j] - c5),
            ] {
                ans = ans.max(t2.min(t5));
            }
        }
    }
    ans
}
fn main() { println!("{}", max_trailing_zeros(vec![vec![23,17,19],vec![8,1,16],vec![7,23,8],vec![1,7,1],vec![11,10,19],vec![11,28,9],vec![18,7,8],vec![26,5,4],vec![22,23,6],vec![32,29,20],vec![32,17,20],vec![32,21,4]])); }
#[cfg(test)]
mod tests {
    use super::max_trailing_zeros;
    #[test]
    fn example_one() {
        assert_eq!(max_trailing_zeros(vec![vec![23,17,19],vec![8,1,16],vec![7,23,8],vec![1,7,1],vec![11,10,19],vec![11,28,9],vec![18,7,8],vec![26,5,4],vec![22,23,6],vec![32,29,20],vec![32,17,20],vec![32,21,4]]), 3);
    }
}''',

1670: r'''pub struct FrontMiddleBackQueue {
    data: Vec<i32>,
}

impl FrontMiddleBackQueue {
    fn new() -> Self { Self { data: vec![] } }
    fn push_front(&mut self, val: i32) { self.data.insert(0, val); }
    fn push_middle(&mut self, val: i32) { self.data.insert(self.data.len() / 2, val); }
    fn push_back(&mut self, val: i32) { self.data.push(val); }
    fn pop_front(&mut self) -> i32 { if self.data.is_empty() { -1 } else { self.data.remove(0) } }
    fn pop_middle(&mut self) -> i32 {
        if self.data.is_empty() { -1 } else { self.data.remove((self.data.len() - 1) / 2) }
    }
    fn pop_back(&mut self) -> i32 { self.data.pop().unwrap_or(-1) }
}
fn main() {
    let mut q = FrontMiddleBackQueue::new();
    q.push_front(1);
    println!("{}", q.pop_back());
}
#[cfg(test)]
mod tests {
    use super::FrontMiddleBackQueue;
    #[test]
    fn example_one() {
        let mut q = FrontMiddleBackQueue::new();
        q.push_front(1); q.push_back(2); q.push_middle(3); q.push_middle(4);
        assert_eq!(q.pop_front(), 1);
        assert_eq!(q.pop_middle(), 3);
        assert_eq!(q.pop_middle(), 4);
        assert_eq!(q.pop_back(), 2);
        assert_eq!(q.pop_front(), -1);
    }
}''',

1676: r'''use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn lowest_common_ancestor(root: &Option<Box<TreeNode>>, nodes: Vec<Option<Box<TreeNode>>>) -> Option<Box<TreeNode>> {
    let targets: HashSet<i32> = nodes.iter().filter_map(|n| n.as_ref().map(|x| x.val)).collect();
    fn dfs(r: &Option<Box<TreeNode>>, t: &HashSet<i32>) -> (i32, Option<i32>) {
        let Some(n) = r else { return (0, None); };
        let (lc, ll) = dfs(&n.left, t);
        let (rc, rl) = dfs(&n.right, t);
        let mut cnt = lc + rc + if t.contains(&n.val) { 1 } else { 0 };
        if ll.is_some() { return (cnt, ll); }
        if rl.is_some() { return (cnt, rl); }
        if cnt == t.len() as i32 { return (cnt, Some(n.val)); }
        (cnt, None)
    }
    let val = dfs(root, &targets).1?;
    Some(Box::new(TreeNode { val, left: None, right: None }))
}
fn main() { println!("{:?}", lowest_common_ancestor(&None, vec![])); }
#[cfg(test)]
mod tests {
    use super::{lowest_common_ancestor, TreeNode};
    fn node(v: i32) -> Option<Box<TreeNode>> {
        Some(Box::new(TreeNode { val: v, left: None, right: None }))
    }
    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode { val: 5, left: Some(Box::new(TreeNode { val: 6, left: None, right: None })), right: Some(Box::new(TreeNode { val: 2, left: None, right: None })) })),
            right: Some(Box::new(TreeNode { val: 1, left: Some(Box::new(TreeNode { val: 0, left: None, right: None })), right: Some(Box::new(TreeNode { val: 8, left: None, right: None })) })),
        }));
        let r = lowest_common_ancestor(&root, vec![node(6), node(2), node(8)]);
        assert_eq!(r.unwrap().val, 3);
    }
}''',

1680: r'''fn concatenated_binary(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut ans = 0i64;
    for i in 1..=n as i64 {
        let bits = 64 - i.leading_zeros() as i64;
        ans = ((ans << bits) + i) % MOD;
    }
    ans as i32
}
fn main() { println!("{}", concatenated_binary(3)); }
#[cfg(test)]
mod tests {
    use super::concatenated_binary;
    #[test]
    fn example_one() { assert_eq!(concatenated_binary(3), 27); }
    #[test]
    fn example_two() { assert_eq!(concatenated_binary(12), 505379714); }
}''',

1681: r'''fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let sz = n / k;
    let mut dp = vec![vec![i32::MAX; 1 << n]; k + 1];
    dp[0][0] = 0;
    for mask in 0usize..(1 << n) {
        if mask.count_ones() as usize % sz != 0 { continue; }
        let mut inc = 0i32;
        let mut seen = [false; 21];
        let mut mn = 21i32;
        let mut mx = 0i32;
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                let v = nums[i];
                if seen[v as usize] { inc = i32::MAX; break; }
                seen[v as usize] = true;
                mn = mn.min(v);
                mx = mx.max(v);
            }
        }
        if inc == i32::MAX { continue; }
        inc = mx - mn;
        for prev in 0usize..mask {
            if prev | mask != mask { continue; }
            if prev.count_ones() as usize % sz != 0 { continue; }
            let parts = mask.count_ones() as usize / sz;
            if parts <= k && dp[parts - 1][prev] != i32::MAX {
                dp[parts][mask] = dp[parts][mask].min(dp[parts - 1][prev] + inc);
            }
        }
    }
    let ans = dp[k][(1 << n) - 1];
    if ans == i32::MAX { -1 } else { ans }
}
fn main() { println!("{}", minimum_incompatibility(vec![1,2,1,4], 2)); }
#[cfg(test)]
mod tests {
    use super::minimum_incompatibility;
    #[test]
    fn example_one() { assert_eq!(minimum_incompatibility(vec![1,2,1,4], 2), 1); }
}''',

1682: r'''fn longest_palindrome(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut ans = 0i32;
    for c1 in 0..26usize {
        for c2 in c1..26 {
            let mut dp = vec![0i32; n];
            for i in 0..n {
                let ch = (s[i] - b'a') as usize;
                if ch != c1 && ch != c2 { continue; }
                dp[i] = 1;
                if i > 0 && (s[i] == s[i - 1] || (s[i - 1] - b'a') as usize == c1 || (s[i - 1] - b'a') as usize == c2) {
                    if (s[i - 1] - b'a') as usize == c1 || (s[i - 1] - b'a') as usize == c2 {
                        dp[i] = dp[i].max(dp[i - 1] + 1);
                    }
                }
                for j in 0..i {
                    if s[i] == s[j] {
                        let inner = if j + 1 <= i - 1 { dp[j + 1] } else { 0 };
                        dp[i] = dp[i].max(inner + 2);
                    }
                }
                if dp[i] >= k { ans = ans.max(dp[i]); }
            }
        }
    }
    ans
}
fn main() { println!("{}", longest_palindrome("abcccq".into(), 2)); }
#[cfg(test)]
mod tests {
    use super::longest_palindrome;
    #[test]
    fn example_one() { assert_eq!(longest_palindrome("abcccq".into(), 2), 4); }
}''',

1686: r'''use std::cmp::Reverse;

fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
    let mut idx: Vec<usize> = (0..alice_values.len()).collect();
    idx.sort_unstable_by_key(|&i| Reverse(alice_values[i] + bob_values[i]));
    let (mut a, mut b) = (0i64, 0i64);
    for (turn, &i) in idx.iter().enumerate() {
        if turn % 2 == 0 { a += alice_values[i] as i64; } else { b += bob_values[i] as i64; }
    }
    if a > b { 1 } else if a < b { -1 } else { 0 }
}
fn main() { println!("{}", stone_game_vi(vec![1,3], vec![2,4])); }
#[cfg(test)]
mod tests {
    use super::stone_game_vi;
    #[test]
    fn example_one() { assert_eq!(stone_game_vi(vec![1,3], vec![2,4]), 1); }
}''',

1689: r'''fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut ans = 0i32;
    let mut j = 0usize;
    for i in 0..nums1.len() {
        while j < nums2.len() && nums2[j] >= nums1[i] {
            j += 1;
        }
        ans = ans.max(j as i32 - i as i32 - 1);
    }
    ans
}
fn main() { println!("{}", max_distance(vec![8,1,5,2,7], vec![7,2,5,1,7])); }
#[cfg(test)]
mod tests {
    use super::max_distance;
    #[test]
    fn example_one() { assert_eq!(max_distance(vec![8,1,5,2,7], vec![7,2,5,1,7]), 6); }
}''',

1701: r'''fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let n = customers.len();
    let mut cur = 0i64;
    let mut total = 0i64;
    for c in &customers {
        cur = cur.max(c[0] as i64) + c[1] as i64;
        total += cur - c[0] as i64;
    }
    total as f64 / n as f64
}
fn main() { println!("{}", average_waiting_time(vec![vec![1,2],vec![2,5],vec![4,3]])); }
#[cfg(test)]
mod tests {
    use super::average_waiting_time;
    #[test]
    fn example_one() { assert!((average_waiting_time(vec![vec![1,2],vec![2,5],vec![4,3]]) - 5.0).abs() < 1e-5); }
}''',

1705: r'''use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::new();
    let mut ans = 0i32;
    let n = apples.len();
    for day in 0..200000 {
        if day < n && apples[day] > 0 {
            heap.push(Reverse((days[day] + day as i32, apples[day])));
        }
        while heap.peek().map(|Reverse((exp, _))| *exp <= day as i32).unwrap_or(false) {
            heap.pop();
        }
        if let Some(Reverse((_, mut cnt))) = heap.pop() {
            ans += 1;
            cnt -= 1;
            if cnt > 0 { heap.push(Reverse((days.get(day).copied().unwrap_or(0) + day as i32, cnt))); }
        } else if day >= n { break; }
    }
    ans
}
fn main() { println!("{}", eaten_apples(vec![1,2,3,5,2], vec![3,1,1,4,2])); }
#[cfg(test)]
mod tests {
    use super::eaten_apples;
    #[test]
    fn example_one() { assert_eq!(eaten_apples(vec![1,2,3,5,2], vec![3,1,1,4,2]), 7); }
}''',

1707: r'''struct TrieNode {
    child: [Option<Box<TrieNode>>; 2],
}

fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_unstable();
    let mut qs: Vec<(i32, i32, usize)> = queries.iter().enumerate().map(|(i, q)| (q[1], q[0], i)).collect();
    qs.sort_unstable();
    let mut root = TrieNode { child: [None, None] };
    let mut ans = vec![-1; queries.len()];
    let mut ni = 0usize;
    for (limit, x, idx) in qs {
        while ni < nums.len() && nums[ni] <= limit {
            let mut node = &mut root;
            for bit in (0..31).rev() {
                let b = ((nums[ni] as u32) >> bit) & 1;
                node = node.child[b as usize].get_or_insert_with(|| Box::new(TrieNode { child: [None, None] }));
            }
            ni += 1;
        }
        if root.child[0].is_none() && root.child[1].is_none() { continue; }
        let mut node = &root;
        let mut xr = 0i32;
        for bit in (0..31).rev() {
            let b = ((x as u32) >> bit) & 1;
            let want = 1 - b;
            if node.child[want as usize].is_some() {
                xr |= ((want ^ b) << bit) as i32;
                node = node.child[want as usize].as_ref().unwrap();
            } else {
                node = node.child[b as usize].as_ref().unwrap();
            }
        }
        ans[idx] = xr;
    }
    ans
}
fn main() { println!("{:?}", maximize_xor(vec![0,1,2,3,4], vec![vec![3,1],vec![3,3]])); }
#[cfg(test)]
mod tests {
    use super::maximize_xor;
    #[test]
    fn example_one() { assert_eq!(maximize_xor(vec![0,1,2,3,4], vec![vec![3,1],vec![3,3]]), vec![3,7]); }
}''',

1708: r'''fn largest_subarray(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    for i in 0..=nums.len() - k {
        let mn = *nums[i..i + k].iter().min().unwrap();
        let mx = *nums[i..i + k].iter().max().unwrap();
        if mx - mn == (k as i32 - 1) {
            return nums[i..i + k].to_vec();
        }
    }
    vec![]
}
fn main() { println!("{:?}", largest_subarray(vec![1,4,5,2,3], 3)); }
#[cfg(test)]
mod tests {
    use super::largest_subarray;
    #[test]
    fn example_one() { assert_eq!(largest_subarray(vec![1,4,5,2,3], 3), vec![5,2,3]); }
}''',

1712: r'''const MOD: i64 = 1_000_000_007;

fn ways_to_split(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut pref = vec![0i64; n + 1];
    for i in 0..n { pref[i + 1] = pref[i] + nums[i] as i64; }
    let total = pref[n];
    let mut ans = 0i64;
    for i in 0..n - 2 {
        let left = pref[i + 1];
        if left * 2 > total { break; }
        let lo = i + 2;
        let hi = n - 1;
        let mut l = lo;
        let mut r = hi;
        while l <= r {
            let mid = (l + r) / 2;
            if pref[mid] - left >= left { r = mid - 1; } else { l = mid + 1; }
        }
        let start = l;
        l = lo; r = hi;
        while l <= r {
            let mid = (l + r) / 2;
            if pref[mid] - left <= total - pref[mid] { l = mid + 1; } else { r = mid - 1; }
        }
        let end = r;
        if start <= end { ans += (end - start + 1) as i64; }
    }
    (ans % MOD) as i32
}
fn main() { println!("{}", ways_to_split(vec![1,1,1])); }
#[cfg(test)]
mod tests {
    use super::ways_to_split;
    #[test]
    fn example_one() { assert_eq!(ways_to_split(vec![1,1,1]), 1); }
}''',

1713: r'''fn min_operations(source: String, target: String) -> i32 {
    let s = source.as_bytes();
    let t = target.as_bytes();
    let mut pos = vec![vec![]; 26];
    for (i, &c) in s.iter().enumerate() {
        pos[(c - b'a') as usize].push(i);
    }
    let mut idx = 0usize;
    let mut matched = 0usize;
    for &c in t {
        let p = &pos[(c - b'a') as usize];
        match p.binary_search(&idx) {
            Ok(i) | Err(i) if i < p.len() => {
                idx = p[i] + 1;
                matched += 1;
            }
            _ => {}
        }
    }
    (t.len() - matched) as i32
}
fn main() { println!("{}", min_operations("abc".into(), "abcbc".into())); }
#[cfg(test)]
mod tests {
    use super::min_operations;
    #[test]
    fn example_one() { assert_eq!(min_operations("abc".into(), "abcbc".into()), 2); }
}''',

1675: r'''use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn minimum_deviation(nums: Vec<i32>) -> i32 {
    let mut maxh = BinaryHeap::new();
    let mut minh = BinaryHeap::new();
    for x in nums {
        maxh.push(x);
        minh.push(Reverse(x));
    }
    let mut ans = *maxh.peek().unwrap() - minh.peek().unwrap().0;
    while maxh.peek().unwrap() > &minh.peek().unwrap().0 {
        let mx = maxh.pop().unwrap();
        ans = ans.min(mx - minh.peek().unwrap().0);
        if mx % 2 != 0 { break; }
        let h = mx / 2;
        maxh.push(h);
        minh.push(Reverse(h));
    }
    ans
}
fn main() { println!("{}", minimum_deviation(vec![1,2,8])); }
#[cfg(test)]
mod tests {
    use super::minimum_deviation;
    #[test]
    fn example_one() { assert_eq!(minimum_deviation(vec![1,2,8]), 3); }
}''',

1695: r'''use std::collections::HashSet;

fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    let mut l = 0usize;
    let mut sum = 0i32;
    let mut ans = 0i32;
    for (r, &x) in nums.iter().enumerate() {
        while !set.insert(x) {
            sum -= nums[l];
            set.remove(&nums[l]);
            l += 1;
        }
        sum += x;
        ans = ans.max(sum);
    }
    ans
}
fn main() { println!("{}", maximum_unique_subarray(vec![4,2,4,5,6])); }
#[cfg(test)]
mod tests {
    use super::maximum_unique_subarray;
    #[test]
    fn example_one() { assert_eq!(maximum_unique_subarray(vec![4,2,4,5,6]), 17); }
}''',

1702: r'''fn maximum_binary_string(binary: String) -> String {
    let n = binary.len();
    let ones = binary.bytes().filter(|&c| c == b'1').count();
    if ones == n { return binary; }
    let mut ans = vec![b'1'; n];
    ans[n - ones - 1] = b'0';
    String::from_utf8(ans).unwrap()
}
fn main() { println!("{}", maximum_binary_string("000110".into())); }
#[cfg(test)]
mod tests {
    use super::maximum_binary_string;
    #[test]
    fn example_one() { assert_eq!(maximum_binary_string("000110".into()), "111011"); }
}''',

1708: r'''fn largest_subarray(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut best = 0usize;
    for i in 0..=nums.len() - k {
        if nums[i..i + k] > nums[best..best + k] {
            best = i;
        }
    }
    nums[best..best + k].to_vec()
}
fn main() { println!("{:?}", largest_subarray(vec![1,4,5,2,3], 3)); }
#[cfg(test)]
mod tests {
    use super::largest_subarray;
    #[test]
    fn example_one() { assert_eq!(largest_subarray(vec![1,4,5,2,3], 3), vec![5,2,3]); }
}''',

1711: r'''const MOD: i64 = 1_000_000_007;

fn count_pairs(deliciousness: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut cnt = HashMap::new();
    let mut ans = 0i64;
    for x in deliciousness {
        let mut p = 1i64;
        while p <= 1 << 21 {
            if let Some(&c) = cnt.get(&(p - x as i64)) {
                ans = (ans + c) % MOD;
            }
            p <<= 1;
        }
        *cnt.entry(x as i64).or_insert(0i64) += 1;
    }
    ans as i32
}
fn main() { println!("{}", count_pairs(vec![1,3,5,7,9])); }
#[cfg(test)]
mod tests {
    use super::count_pairs;
    #[test]
    fn example_one() { assert_eq!(count_pairs(vec![1,3,5,7,9]), 4); }
}''',

1662: r'''fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
    word1.concat() == word2.concat()
}
fn main() { println!("{}", array_strings_are_equal(vec!["ab".into(),"c".into()], vec!["a".into(),"bc".into()])); }
#[cfg(test)]
mod tests {
    use super::array_strings_are_equal;
    #[test]
    fn example_one() { assert!(array_strings_are_equal(vec!["ab".into(),"c".into()], vec!["a".into(),"bc".into()])); }
    #[test]
    fn example_two() { assert!(!array_strings_are_equal(vec!["a".into(),"b".into()], vec!["a".into(),"b".into(),"c".into()])); }
}''',

1668: r'''fn trailing(x: i32) -> (i32, i32) {
    let mut a = 0i32;
    let mut b = 0i32;
    let mut v = x;
    while v > 0 && v % 2 == 0 { a += 1; v /= 2; }
    v = x;
    while v > 0 && v % 5 == 0 { b += 1; v /= 5; }
    (a, b)
}

fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut row2 = vec![vec![0i32; n + 1]; m + 1];
    let mut row5 = row2.clone();
    let mut col2 = row2.clone();
    let mut col5 = row2.clone();
    for i in 0..m {
        for j in 0..n {
            let (a, b) = trailing(grid[i][j]);
            row2[i + 1][j + 1] = row2[i + 1][j] + a;
            row5[i + 1][j + 1] = row5[i + 1][j] + b;
        }
    }
    for j in 0..n {
        for i in 0..m {
            let (a, b) = trailing(grid[i][j]);
            col2[i + 1][j + 1] = col2[i][j + 1] + a;
            col5[i + 1][j + 1] = col5[i][j + 1] + b;
        }
    }
    let mut ans = 0i32;
    for i in 0..m {
        for j in 0..n {
            let (c2, c5) = trailing(grid[i][j]);
            let t2 = row2[i + 1][j + 1] + col2[i + 1][j + 1] - c2;
            let t5 = row5[i + 1][j + 1] + col5[i + 1][j + 1] - c5;
            ans = ans.max(t2.min(t5));
            let t2 = row2[i + 1][n] - row2[i + 1][j + 1] + col2[m][j + 1] - col2[i + 1][j + 1] + c2;
            let t5 = row5[i + 1][n] - row5[i + 1][j + 1] + col5[m][j + 1] - col5[i + 1][j + 1] + c5;
            ans = ans.max(t2.min(t5));
            let t2 = row2[i + 1][j + 1] + col2[m][j + 1] - col2[i + 1][j + 1] - c2;
            let t5 = row5[i + 1][j + 1] + col5[m][j + 1] - col5[i + 1][j + 1] - c5;
            ans = ans.max(t2.min(t5));
            let t2 = row2[m][j + 1] - row2[i + 1][j + 1] + col2[i + 1][j + 1] - c2;
            let t5 = row5[m][j + 1] - row5[i + 1][j + 1] + col5[i + 1][j + 1] - c5;
            ans = ans.max(t2.min(t5));
        }
    }
    ans
}
fn main() { println!("{}", max_trailing_zeros(vec![vec![23,17,19],vec![8,1,16],vec![7,23,8],vec![1,7,1],vec![11,10,19],vec![11,28,9],vec![18,7,8],vec![26,5,4],vec![22,23,6],vec![32,29,20],vec![32,17,20],vec![32,21,4]])); }
#[cfg(test)]
mod tests {
    use super::max_trailing_zeros;
    #[test]
    fn example_one() {
        assert_eq!(max_trailing_zeros(vec![vec![23,17,19],vec![8,1,16],vec![7,23,8],vec![1,7,1],vec![11,10,19],vec![11,28,9],vec![18,7,8],vec![26,5,4],vec![22,23,6],vec![32,29,20],vec![32,17,20],vec![32,21,4]]), 3);
    }
}''',

1681: r'''fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let sz = n / k;
    let mut memo = std::collections::HashMap::new();
    fn dfs(nums: &[i32], k: i32, sz: usize, mask: i32, memo: &mut std::collections::HashMap<(i32, i32), i32>) -> i32 {
        if mask == (1 << nums.len()) - 1 { return if k == 0 { 0 } else { i32::MAX }; }
        if k == 0 { return i32::MAX; }
        if let Some(&v) = memo.get(&(mask, k)) { return v; }
        let mut best = i32::MAX;
        let mut sub = mask;
        while sub < (1 << nums.len()) {
            sub = (sub + 1) | mask;
            if sub == mask { continue; }
            if (sub & mask) != mask { continue; }
            if sub.count_ones() as usize != sz { continue; }
            let mut seen = [false; 21];
            let mut mn = 21i32;
            let mut mx = 0i32;
            let mut ok = true;
            for i in 0..nums.len() {
                if (sub >> i) & 1 == 0 { continue; }
                if seen[nums[i] as usize] { ok = false; break; }
                seen[nums[i] as usize] = true;
                mn = mn.min(nums[i]);
                mx = mx.max(nums[i]);
            }
            if !ok { continue; }
            let rest = dfs(nums, k - 1, sz, sub, memo);
            if rest != i32::MAX { best = best.min(mx - mn + rest); }
        }
        memo.insert((mask, k), best);
        best
    }
    let ans = dfs(&nums, k as i32, sz, 0, &mut memo);
    if ans == i32::MAX { -1 } else { ans }
}
fn main() { println!("{}", minimum_incompatibility(vec![1,2,1,4], 2)); }
#[cfg(test)]
mod tests {
    use super::minimum_incompatibility;
    #[test]
    fn example_one() { assert_eq!(minimum_incompatibility(vec![1,2,1,4], 2), 1); }
}''',

1682: r'''fn longest_palindrome(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut ans = 0i32;
    for c1 in 0..26usize {
        for c2 in c1..26 {
            let mut dp = vec![0i32; n];
            for i in 0..n {
                let ch = (s[i] - b'a') as usize;
                if ch != c1 && ch != c2 { continue; }
                dp[i] = 1;
                if i > 0 && ((s[i - 1] - b'a') as usize == c1 || (s[i - 1] - b'a') as usize == c2) {
                    dp[i] = dp[i].max(dp[i - 1] + 1);
                }
                for j in 0..i {
                    if s[i] == s[j] {
                        let inner = if j + 1 <= i - 1 { dp[j + 1] } else { 0 };
                        if (s[j] - b'a') as usize == c1 || (s[j] - b'a') as usize == c2 {
                            dp[i] = dp[i].max(inner + 2);
                        }
                    }
                }
                if dp[i] >= k { ans = ans.max(dp[i]); }
            }
        }
    }
    ans
}
fn main() { println!("{}", longest_palindrome("abcccq".into(), 2)); }
#[cfg(test)]
mod tests {
    use super::longest_palindrome;
    #[test]
    fn example_one() { assert_eq!(longest_palindrome("abcccq".into(), 2), 4); }
}''',
}
