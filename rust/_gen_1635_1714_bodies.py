"""Solution bodies for _gen_1635_1714.py."""

BODIES: dict[int, str] = {
1636: r'''fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut cnt = [0i32; 2001];
    for &x in &nums { cnt[(x + 1000) as usize] += 1; }
    let mut v = nums;
    v.sort_by_key(|&x| (cnt[(x + 1000) as usize], -x));
    v
}
fn main() { println!("{:?}", frequency_sort(vec![1,1,2,2,2,3])); }
#[cfg(test)]
mod tests {
    use super::frequency_sort;
    #[test]
    fn example_one() { assert_eq!(frequency_sort(vec![1,1,2,2,2,3]), vec![3,1,1,2,2,2]); }
}''',

1637: r'''fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    let mut xs: Vec<i32> = points.iter().map(|p| p[0]).collect();
    xs.sort_unstable();
    let mut ans = 0i32;
    for w in xs.windows(2) { ans = ans.max(w[1] - w[0]); }
    ans
}
fn main() { println!("{}", max_width_of_vertical_area(vec![vec![8,7],vec![9,9],vec![7,4],vec![9,7]])); }
#[cfg(test)]
mod tests {
    use super::max_width_of_vertical_area;
    #[test]
    fn example_one() { assert_eq!(max_width_of_vertical_area(vec![vec![8,7],vec![9,9],vec![7,4],vec![9,7]]), 3); }
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
    fn example_one() { assert_eq!(count_substrings("aba".into(), "baba".into()), 1); }
    #[test]
    fn example_two() { assert_eq!(count_substrings("ab".into(), "bb".into()), 3); }
}''',

1639: r'''const MOD: i64 = 1_000_000_007;

fn num_ways(words: Vec<String>, target: String) -> i32 {
    let words: Vec<Vec<u8>> = words.iter().map(|w| w.as_bytes().to_vec()).collect();
    let t = target.as_bytes();
    let m = t.len();
    let n = words.len();
    let mut col = vec![vec![0i64; 26]; n];
    for j in 0..n {
        for w in &words {
            if j < w.len() { col[j][(w[j] - b'a') as usize] += 1; }
        }
    }
    let mut dp = vec![0i64; m + 1];
    dp[0] = 1;
    for j in 0..n {
        let mut nd = dp.clone();
        for i in 0..m {
            let c = (t[i] - b'a') as usize;
            nd[i + 1] = (nd[i + 1] + dp[i] * col[j][c]) % MOD;
        }
        dp = nd;
    }
    dp[m] as i32
}
fn main() { println!("{}", num_ways(vec!["acca".into(),"bbbb".into()], "aba".into())); }
#[cfg(test)]
mod tests {
    use super::num_ways;
    #[test]
    fn example_one() { assert_eq!(num_ways(vec!["acca".into(),"bbbb".into()], "aba".into()), 6); }
}''',

1640: r'''use std::collections::HashMap;

fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
    let mut pos = HashMap::new();
    for (i, &x) in arr.iter().enumerate() { pos.insert(x, i); }
    for p in &pieces {
        let start = match pos.get(&p[0]) { Some(&s) => s, None => return false };
        if start + p.len() > arr.len() { return false; }
        if arr[start..start + p.len()] != p[..] { return false; }
    }
    true
}
fn main() { println!("{}", can_form_array(vec![15,88], vec![vec![88]])); }
#[cfg(test)]
mod tests {
    use super::can_form_array;
    #[test]
    fn example_one() { assert!(can_form_array(vec![15,88], vec![vec![88]])); }
    #[test]
    fn example_two() { assert!(!can_form_array(vec![49,18,16], vec![vec![16,18,49]])); }
}''',

1641: r'''fn letter(r: i32, c: i32) -> u8 { (b'a' + ((r + c - 2) % 26) as u8) }

fn is_sorted(r: i32, c: i32) -> bool {
    let ch = letter(r, c);
    for k in 1..c {
        if letter(r, k) > ch { return false; }
    }
    for k in 1..r {
        if letter(k, c) > ch { return false; }
    }
    true
}

fn count_sorted_squares(coordinates: Vec<Vec<i32>>) -> i32 {
    coordinates.iter().filter(|p| is_sorted(p[0], p[1])).count() as i32
}
fn main() { println!("{}", count_sorted_squares(vec![vec![1,1],vec![8,8]])); }
#[cfg(test)]
mod tests {
    use super::count_sorted_squares;
    #[test]
    fn example_one() { assert_eq!(count_sorted_squares(vec![vec![1,1],vec![8,8],vec![5,6]]), 3); }
}''',

1642: r'''use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
    let mut heap = BinaryHeap::new();
    let mut b = bricks as i64;
    let mut lad = ladders;
    let mut pos = 0usize;
    for i in 0..heights.len().saturating_sub(1) {
        let diff = heights[i + 1] - heights[i];
        if diff <= 0 { pos = i + 1; continue; }
        heap.push(Reverse(diff));
        b -= diff as i64;
        if b < 0 {
            if lad == 0 { break; }
            lad -= 1;
            if let Some(Reverse(x)) = heap.pop() { b += x as i64; }
        }
        pos = i + 1;
    }
    pos as i32
}
fn main() { println!("{}", furthest_building(vec![4,2,7,6,9,14,12], 5, 1)); }
#[cfg(test)]
mod tests {
    use super::furthest_building;
    #[test]
    fn example_one() { assert_eq!(furthest_building(vec![4,2,7,6,9,14,12], 5, 1), 4); }
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
        let right = comb(r + c - 1, c - 1);
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
    fn dfs(r: &Option<Box<TreeNode>>, p: i32, q: i32) -> (bool, bool, Option<Box<TreeNode>>) {
        let Some(n) = r else { return (false, false, None); };
        let (lp, lq, ll) = dfs(&n.left, p, q);
        let (rp, rq, rl) = dfs(&n.right, p, q);
        let has_p = lp || n.val == p;
        let has_q = lq || n.val == q;
        if ll.is_some() { return (has_p, has_q, ll); }
        if rl.is_some() { return (has_p, has_q, rl); }
        if has_p && has_q {
            return (true, true, Some(TreeNode { val: n.val, left: None, right: None }));
        }
        (has_p, has_q, None)
    }
    dfs(root, p, q).2
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
        let root = build(vec![Some(3),Some(5),Some(1),Some(6),Some(2),Some(0),Some(8),None,None,Some(7),Some(4)]);
        let p = build(vec![Some(5)]);
        let q = build(vec![Some(1)]);
        assert_eq!(lowest_common_ancestor(&root, &p, &q).unwrap().val, 3);
    }
}''',

1646: r'''fn get_maximum_generated(n: i32) -> i32 {
    if n == 0 { return 0; }
    let n = n as usize;
    let mut a = vec![0i32; n + 1];
    a[1] = 1;
    for i in 2..=n {
        a[i] = if i % 2 == 0 { a[i / 2] } else { a[i / 2] + a[i / 2 + 1] };
    }
    *a.iter().max().unwrap()
}
fn main() { println!("{}", get_maximum_generated(7)); }
#[cfg(test)]
mod tests {
    use super::get_maximum_generated;
    #[test]
    fn example_one() { assert_eq!(get_maximum_generated(7), 3); }
    #[test]
    fn example_two() { assert_eq!(get_maximum_generated(2), 1); }
}''',

1647: r'''use std::collections::HashSet;

fn min_deletions(s: String) -> i32 {
    let mut cnt = [0i32; 26];
    for c in s.bytes() { cnt[(c - b'a') as usize] += 1; }
    let mut freqs: Vec<i32> = cnt.into_iter().filter(|&x| x > 0).collect();
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

1648: r'''fn max_profit(inventory: Vec<i32>, orders: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut inv = inventory;
    inv.sort_unstable_by(|a, b| b.cmp(a));
    let mut rem = orders as i64;
    let mut ans = 0i64;
    let mut i = 0usize;
    while i < inv.len() && rem > 0 {
        let j = (i + 1..=inv.len()).find(|&j| j == inv.len() || inv[j] < inv[i]).unwrap();
        let cnt = (j - i) as i64;
        let hi = inv[i] as i64;
        let lo = if j == inv.len() { 0 } else { inv[j] as i64 };
        let span = hi - lo;
        let take = rem.min(cnt * span);
        let full = take / cnt;
        let rest = take % cnt;
        ans = (ans + cnt * full * (2 * hi - full + 1) / 2) % MOD;
        ans = (ans + rest * (2 * hi - full - 2 * rest + 1) / 2) % MOD;
        rem -= take;
        i = j;
    }
    ans as i32
}
fn main() { println!("{}", max_profit(vec![2,5], 4)); }
#[cfg(test)]
mod tests {
    use super::max_profit;
    #[test]
    fn example_one() { assert_eq!(max_profit(vec![2,5], 4), 14); }
}''',

1649: r'''struct Fenwick {
    n: usize,
    bit: Vec<i32>,
}
impl Fenwick {
    fn new(n: usize) -> Self { Self { n, bit: vec![0; n + 1] } }
    fn add(&mut self, mut i: usize, v: i32) {
        i += 1;
        while i <= self.n {
            self.bit[i] += v;
            i += i & i.wrapping_neg();
        }
    }
    fn sum(&self, mut i: usize) -> i32 {
        let mut s = 0i32;
        i += 1;
        while i > 0 {
            s += self.bit[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
}

fn create_sorted_array(instructions: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut fw = Fenwick::new(100_001);
    let mut ans = 0i64;
    for x in instructions {
        let x = x as usize;
        let less = fw.sum(x.saturating_sub(1)) as i64;
        let total = fw.sum(100_000) as i64;
        let greater = total - fw.sum(x) as i64;
        ans = (ans + less.min(greater)) % MOD;
        fw.add(x, 1);
    }
    ans as i32
}
fn main() { println!("{}", create_sorted_array(vec![1,5,6,2])); }
#[cfg(test)]
mod tests {
    use super::create_sorted_array;
    #[test]
    fn example_one() { assert_eq!(create_sorted_array(vec![1,5,6,2]), 1); }
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
    fn example_one() { assert_eq!(decrypt(vec![5,7,1,4], 3), vec![12,10,1,11]); }
    #[test]
    fn example_two() { assert_eq!(decrypt(vec![1,2,3,4], 0), vec![0,0,0,0]); }
}''',

1653: r'''fn minimum_deletions(s: String) -> i32 {
    let mut b = 0i32;
    let mut ans = 0i32;
    for c in s.bytes() {
        if c == b'a' { ans += b; } else { b += 1; }
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

1654: r'''use std::collections::HashSet;

fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
    let mut ban: HashSet<i32> = forbidden.into_iter().collect();
    ban.insert(0);
    let mut q = std::collections::VecDeque::from([(0i32, 0i32, 0i32)]);
    let mut seen: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    while let Some((pos, back, steps)) = q.pop_front() {
        if pos == x { return steps; }
        for (np, nback) in [(pos + a, 0), (pos - b, 1)] {
            if np < 0 || np > 6000 || ban.contains(&np) { continue; }
            if nback == 1 && back == 1 { continue; }
            if seen.insert((np, nback)) { q.push_back((np, nback, steps + 1)); }
        }
    }
    -1
}
fn main() { println!("{}", minimum_jumps(vec![14,4,18,1,15], 3, 15, 9)); }
#[cfg(test)]
mod tests {
    use super::minimum_jumps;
    #[test]
    fn example_one() { assert_eq!(minimum_jumps(vec![14,4,18,1,15], 3, 15, 9), 3); }
}''',

1655: r'''fn can_distribute(nums: Vec<i32>, quantity: Vec<i32>) -> bool {
    let mut cnt = [0i32; 101];
    for x in nums { cnt[x as usize] += 1; }
    let mut qty = quantity;
    qty.sort_unstable_by(|a, b| b.cmp(a));
    fn dfs(cnt: &mut [i32; 101], qty: &[i32], i: usize) -> bool {
        if i == qty.len() { return true; }
        let need = qty[i];
        for v in 1..=100 {
            if cnt[v] >= need {
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

1656: r'''pub struct OrderedStream {
    n: i32,
    ptr: i32,
    stream: Vec<String>,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        OrderedStream { n, ptr: 1, stream: vec![String::new(); n as usize + 1] }
    }
    fn insert(&mut self, id: i32, value: String) -> Vec<String> {
        self.stream[id as usize] = value;
        let mut ans = vec![];
        while self.ptr <= self.n && !self.stream[self.ptr as usize].is_empty() {
            ans.push(self.stream[self.ptr as usize].clone());
            self.ptr += 1;
        }
        ans
    }
}
fn main() {
    let mut os = OrderedStream::new(5);
    println!("{:?}", os.insert(3, "ccccc".into()));
}
#[cfg(test)]
mod tests {
    use super::OrderedStream;
    #[test]
    fn example_one() {
        let mut os = OrderedStream::new(5);
        assert_eq!(os.insert(3, "ccccc".into()), Vec::<String>::new());
        assert_eq!(os.insert(1, "aaaaa".into()), vec!["aaaaa"]);
        assert_eq!(os.insert(2, "bbbbb".into()), vec!["bbbbb", "ccccc"]);
    }
}''',

1657: r'''fn close_strings(word1: String, word2: String) -> bool {
    let mut c1 = [0i32; 26];
    let mut c2 = [0i32; 26];
    for c in word1.bytes() { c1[(c - b'a') as usize] += 1; }
    for c in word2.bytes() { c2[(c - b'a') as usize] += 1; }
    let mut f1: Vec<i32> = c1.into_iter().filter(|&x| x > 0).collect();
    let mut f2: Vec<i32> = c2.into_iter().filter(|&x| x > 0).collect();
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

1658: r'''use std::collections::HashMap;

fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
    let total: i32 = nums.iter().sum();
    let need = total - x;
    if need < 0 { return -1; }
    if need == 0 { return nums.len() as i32; }
    let mut best = -1i32;
    let mut sum = 0i32;
    let mut left = 0usize;
    for (right, &v) in nums.iter().enumerate() {
        sum += v;
        while sum > need && left <= right {
            sum -= nums[left];
            left += 1;
        }
        if sum == need { best = best.max((right - left + 1) as i32); }
    }
    if best < 0 { -1 } else { nums.len() as i32 - best }
}
fn main() { println!("{}", min_operations(vec![1,1,4,2,3], 5)); }
#[cfg(test)]
mod tests {
    use super::min_operations;
    #[test]
    fn example_one() { assert_eq!(min_operations(vec![1,1,4,2,3], 5), 2); }
    #[test]
    fn example_two() { assert_eq!(min_operations(vec![5,6,7,8,9], 4), -1); }
}''',

1659: r'''fn seat_gain(row: usize, mask: i32, prev: i32, seats: &[Vec<i32>], intro: bool) -> i32 {
    let n = seats[0].len();
    let mut g = if intro { 120 } else { 40 };
    for j in 0..n {
        if (mask >> j) & 1 == 0 { continue; }
        if seats[row][j] == 0 { return i32::MIN / 4; }
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
        for prev in 0..(1 << n) {
            for i in 0..=intro {
                for e in 0..=extro {
                    if dp[prev][i][e] <= i32::MIN / 8 { continue; }
                    for mask in 0..(1 << n) {
                        let mut ni = i;
                        let mut ne = e;
                        let mut gain = 0i32;
                        let mut ok = true;
                        for j in 0..n {
                            if (mask >> j) & 1 == 0 { continue; }
                            if seats[row][j] == 0 { ok = false; break; }
                        }
                        if !ok { continue; }
                        for j in 0..n {
                            if (mask >> j) & 1 == 0 { continue; }
                            let sub = 1 << j;
                            let only = mask & sub;
                            let g = seat_gain(row, only, prev as i32, &seats, ni < intro);
                            if g <= i32::MIN / 8 { ok = false; break; }
                            gain += g;
                            if ni < intro { ni += 1; } else { ne += 1; }
                            if ne > extro { ok = false; break; }
                        }
                        if !ok || ni > intro || ne > extro { continue; }
                        nd[mask][ni][ne] = nd[mask][ni][ne].max(dp[prev][i][e] + gain);
                    }
                }
            }
        }
        dp = nd;
    }
    (0..(1 << n)).map(|mask| dp[mask][intro][extro]).max().unwrap_or(0)
}
fn main() { println!("{}", get_max_happy(vec![1,1], vec![vec![1,1],vec![1,1]])); }
#[cfg(test)]
mod tests {
    use super::get_max_happy;
    #[test]
    fn example_one() { assert_eq!(get_max_happy(vec![1,1], vec![vec![1,1],vec![1,1]]), 4); }
}''',

1660: r'''use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn correct_tree(root: Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
    let mut seen = HashMap::new();
    let mut dup = None;
    fn mark(node: &Option<Box<TreeNode>>, seen: &mut HashMap<i32, i32>, dup: &mut Option<i32>) {
        let Some(n) = node else { return; };
        if seen.insert(n.val, 1).is_some() { *dup = Some(n.val); }
        mark(&n.left, seen, dup);
        mark(&n.right, seen, dup);
    }
    mark(&root, &mut seen, &mut dup);
    let dup = dup?;
    fn fix(node: &mut Option<Box<TreeNode>>, dup: i32) -> bool {
        let Some(n) = node else { return false; };
        if fix(&mut n.left, dup) || fix(&mut n.right, dup) { return true; }
        if n.val == dup {
            if let Some(l) = n.left.take() {
                if l.val == dup { n.left = None; return true; }
            }
            if let Some(r) = n.right.take() {
                if r.val == dup { n.right = None; return true; }
            }
        }
        false
    }
    let mut root = root;
    fix(&mut root, dup);
    root
}
fn main() { println!("{:?}", correct_tree(None)); }
#[cfg(test)]
mod tests {
    use super::{correct_tree, TreeNode};
    #[test]
    fn example_one() {
        let root = Some(Box::new(TreeNode {
            val: 5,
            left: Some(Box::new(TreeNode { val: 2, left: None, right: None })),
            right: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode { val: 1, left: None, right: None })),
                right: None,
            })),
        }));
        let r = correct_tree(root);
        assert!(r.is_some());
    }
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
    fn example_two() { assert!(!array_strings_are_equal(vec!["a".into(),"b".into()], vec!["a".into(),"b".into()])); }
}''',

1663: r'''fn get_smallest_string(n: i32, k: i32) -> String {
    let mut rem = k - n;
    let mut ans = vec![b'a'; n as usize];
    for i in (0..n as usize).rev() {
        let add = rem.min(25);
        ans[i] += add as u8;
        rem -= add;
    }
    String::from_utf8(ans).unwrap()
}
fn main() { println!("{}", get_smallest_string(3, 27)); }
#[cfg(test)]
mod tests {
    use super::get_smallest_string;
    #[test]
    fn example_one() { assert_eq!(get_smallest_string(3, 27), "aay"); }
}''',

1664: r'''fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut odd = 0i64;
    let mut even = 0i64;
    for (i, &x) in nums.iter().enumerate() {
        if i % 2 == 0 { even += x as i64; } else { odd += x as i64; }
    }
    let mut ans = 0i32;
    let (mut lo, mut le) = (0i64, 0i64);
    for i in 0..n {
        let ro = odd - lo - if i % 2 == 1 { nums[i] as i64 } else { 0 };
        let re = even - le - if i % 2 == 0 { nums[i] as i64 } else { 0 };
        let (lodd, leven) = if i % 2 == 0 { (lo, le + nums[i] as i64) } else { (lo + nums[i] as i64, le) };
        let (rodd, reven) = if i % 2 == 0 { (ro, re) } else { (ro + nums[i] as i64, re) };
        if lodd + rodd == leven + reven { ans += 1; }
        if i % 2 == 0 { le += nums[i] as i64; } else { lo += nums[i] as i64; }
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

1665: r'''fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
    let mut t = tasks;
    t.sort_unstable_by_key(|v| v[1]);
    let mut energy = 0i32;
    for v in t {
        if energy + v[0] < v[1] { energy = v[1]; } else { energy += v[0]; }
    }
    energy
}
fn main() { println!("{}", minimum_effort(vec![vec![1,2],vec![2,4],vec![4,8]])); }
#[cfg(test)]
mod tests {
    use super::minimum_effort;
    #[test]
    fn example_one() { assert_eq!(minimum_effort(vec![vec![1,2],vec![2,4],vec![4,8]]), 8); }
}''',

1666: r'''#[derive(Clone)]
pub struct Node {
    pub val: i32,
    pub children: Vec<Node>,
}

fn flip_binary_tree(root: Option<Box<Node>>, leaf: i32) -> Option<Box<Node>> {
    fn find_path(node: &Node, leaf: i32, path: &mut Vec<i32>) -> bool {
        path.push(node.val);
        if node.val == leaf && node.children.is_empty() { return true; }
        for c in &node.children {
            if find_path(c, leaf, path) { return true; }
        }
        path.pop();
        false
    }
    let root = root?;
    let mut path = vec![];
    find_path(root.as_ref(), leaf, &mut path);
    let mut cur = root;
    for i in (1..path.len()).rev() {
        let parent_val = path[i - 1];
        let child_val = path[i];
        fn rewire(node: &mut Node, parent_val: i32, child_val: i32) {
            if node.val == child_val {
                node.children.retain(|c| c.val != parent_val);
                return;
            }
            for c in &mut node.children { rewire(c, parent_val, child_val); }
        }
        rewire(cur.as_mut(), parent_val, child_val);
        let mut new_root = None;
        fn take_child(node: &mut Node, val: i32) -> Option<Box<Node>> {
            for (i, c) in node.children.iter().enumerate() {
                if c.val == val {
                    let child = node.children.remove(i);
                    return Some(Box::new(child));
                }
            }
            for c in &mut node.children {
                if let Some(x) = take_child(c, val) { return Some(x); }
            }
            None
        }
        if let Some(mut nr) = take_child(cur.as_mut(), child_val) {
            nr.children.push(*cur);
            cur = nr;
        }
    }
    Some(cur)
}
fn main() { println!("{:?}", flip_binary_tree(None, 0)); }
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
    let mut tl5 = vec![vec![0i32; n]; m];
    let mut tr2 = vec![vec![0i32; n]; m];
    let mut tr5 = vec![vec![0i32; n]; m];
    let mut bl2 = vec![vec![0i32; n]; m];
    let mut bl5 = vec![vec![0i32; n]; m];
    let mut br2 = vec![vec![0i32; n]; m];
    let mut br5 = vec![vec![0i32; n]; m];
    for i in 0..m {
        for j in 0..n {
            let (a, b) = trailing(grid[i][j]);
            tl2[i][j] = a + if i > 0 { tl2[i - 1][j] } else { 0 } + if j > 0 { tl2[i][j - 1] } else { 0 } - if i > 0 && j > 0 { tl2[i - 1][j - 1] } else { 0 };
            tl5[i][j] = b + if i > 0 { tl5[i - 1][j] } else { 0 } + if j > 0 { tl5[i][j - 1] } else { 0 } - if i > 0 && j > 0 { tl5[i - 1][j - 1] } else { 0 };
        }
    }
    for i in (0..m).rev() {
        for j in 0..n {
            let (a, b) = trailing(grid[i][j]);
            bl2[i][j] = a + if i + 1 < m { bl2[i + 1][j] } else { 0 } + if j > 0 { bl2[i][j - 1] } else { 0 } - if i + 1 < m && j > 0 { bl2[i + 1][j - 1] } else { 0 };
            bl5[i][j] = b + if i + 1 < m { bl5[i + 1][j] } else { 0 } + if j > 0 { bl5[i][j - 1] } else { 0 } - if i + 1 < m && j > 0 { bl5[i + 1][j - 1] } else { 0 };
        }
    }
    for i in 0..m {
        for j in (0..n).rev() {
            let (a, b) = trailing(grid[i][j]);
            tr2[i][j] = a + if i > 0 { tr2[i - 1][j] } else { 0 } + if j + 1 < n { tr2[i][j + 1] } else { 0 } - if i > 0 && j + 1 < n { tr2[i - 1][j + 1] } else { 0 };
            tr5[i][j] = b + if i > 0 { tr5[i - 1][j] } else { 0 } + if j + 1 < n { tr5[i][j + 1] } else { 0 } - if i > 0 && j + 1 < n { tr5[i - 1][j + 1] } else { 0 };
        }
    }
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            let (a, b) = trailing(grid[i][j]);
            br2[i][j] = a + if i + 1 < m { br2[i + 1][j] } else { 0 } + if j + 1 < n { br2[i][j + 1] } else { 0 } - if i + 1 < m && j + 1 < n { br2[i + 1][j + 1] } else { 0 };
            br5[i][j] = b + if i + 1 < m { br5[i + 1][j] } else { 0 } + if j + 1 < n { br5[i][j + 1] } else { 0 } - if i + 1 < m && j + 1 < n { br5[i + 1][j + 1] } else { 0 };
        }
    }
    let mut ans = 0i32;
    for i in 0..m {
        for j in 0..n {
            let paths = [
                (tl2[i][j] + br2[i][j] - trailing(grid[i][j]).0, tl5[i][j] + br5[i][j] - trailing(grid[i][j]).1),
                (tl2[i][j] + bl2[i][j] - trailing(grid[i][j]).0, tl5[i][j] + bl5[i][j] - trailing(grid[i][j]).1),
                (tr2[i][j] + bl2[i][j] - trailing(grid[i][j]).0, tr5[i][j] + bl5[i][j] - trailing(grid[i][j]).1),
                (tr2[i][j] + br2[i][j] - trailing(grid[i][j]).0, tr5[i][j] + br5[i][j] - trailing(grid[i][j]).1),
            ];
            for (t2, t5) in paths {
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

1669: r'''#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn merge_in_between(list1: Option<Box<ListNode>>, a: i32, b: i32, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: list1 });
    let mut cur = dummy.as_mut();
    for _ in 0..a { cur = cur.next.as_mut().unwrap(); }
    let mut tail = cur.next.as_mut().unwrap();
    for _ in 0..(b - a) { tail = tail.next.as_mut().unwrap(); }
    let rest = tail.next.take();
    cur.next = list2;
    while cur.next.is_some() { cur = cur.next.as_mut().unwrap(); }
    cur.next = rest;
    dummy.next
}
fn main() { println!("{:?}", merge_in_between(None, 0, 0, None)); }
#[cfg(test)]
mod tests {
    use super::{merge_in_between, ListNode};
    fn build(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &x in v.iter().rev() { head = Some(Box::new(ListNode { val: x, next: head })); }
        head
    }
    fn vals(mut h: Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = vec![];
        while let Some(n) = h { out.push(n.val); h = n.next; }
        out
    }
    #[test]
    fn example_one() {
        let l1 = build(vec![10,1,13,6,9,5]);
        let l2 = build(vec![1000000,1000001,1000002]);
        let r = merge_in_between(l1, 3, 4, l2);
        assert_eq!(vals(r), vec![10,1,13,1000000,1000001,1000002,5]);
    }
}''',

1670: r'''use std::collections::VecDeque;

pub struct FrontMiddleBackQueue {
    left: VecDeque<i32>,
    right: VecDeque<i32>,
}

impl FrontMiddleBackQueue {
    fn new() -> Self { Self { left: VecDeque::new(), right: VecDeque::new() } }
    fn balance(&mut self) {
        while self.left.len() > self.right.len() + 1 {
            if let Some(x) = self.left.pop_back() { self.right.push_front(x); }
        }
        while self.right.len() > self.left.len() {
            if let Some(x) = self.right.pop_front() { self.left.push_back(x); }
        }
    }
    fn push_front(&mut self, val: i32) { self.left.push_front(val); self.balance(); }
    fn push_middle(&mut self, val: i32) {
        if self.left.len() <= self.right.len() { self.left.push_back(val); } else { self.right.push_front(val); }
        self.balance();
    }
    fn push_back(&mut self, val: i32) { self.right.push_back(val); self.balance(); }
    fn pop_front(&mut self) -> i32 {
        let v = self.left.pop_front().unwrap_or_else(|| self.right.pop_front().unwrap_or(-1));
        self.balance();
        v
    }
    fn pop_middle(&mut self) -> i32 {
        let v = if self.left.len() > self.right.len() {
            self.left.pop_back().unwrap_or(-1)
        } else {
            self.right.pop_front().unwrap_or(-1)
        };
        self.balance();
        v
    }
    fn pop_back(&mut self) -> i32 {
        let v = self.right.pop_back().unwrap_or_else(|| self.left.pop_back().unwrap_or(-1));
        self.balance();
        v
    }
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

1671: r'''fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut lis = vec![1i32; n];
    for i in 0..n {
        for j in 0..i {
            if nums[j] < nums[i] { lis[i] = lis[i].max(lis[j] + 1); }
        }
    }
    let mut lds = vec![1i32; n];
    for i in (0..n).rev() {
        for j in (i + 1..n).rev() {
            if nums[j] < nums[i] { lds[i] = lds[i].max(lds[j] + 1); }
        }
    }
    let mut best = 0i32;
    for i in 0..n {
        if lis[i] > 1 && lds[i] > 1 { best = best.max(lis[i] + lds[i] - 1); }
    }
    n as i32 - best
}
fn main() { println!("{}", minimum_mountain_removals(vec![1,3,1])); }
#[cfg(test)]
mod tests {
    use super::minimum_mountain_removals;
    #[test]
    fn example_one() { assert_eq!(minimum_mountain_removals(vec![1,3,1]), 0); }
    #[test]
    fn example_two() { assert_eq!(minimum_mountain_removals(vec![2,1,1,5,6,2,3,1]), 3); }
}''',

1672: r'''fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.iter().map(|a| a.iter().sum()).max().unwrap_or(0)
}
fn main() { println!("{}", maximum_wealth(vec![vec![1,2,3],vec![3,2,1]])); }
#[cfg(test)]
mod tests {
    use super::maximum_wealth;
    #[test]
    fn example_one() { assert_eq!(maximum_wealth(vec![vec![1,2,3],vec![3,2,1]]), 6); }
}''',

1673: r'''fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let mut ans = vec![];
    for (i, &x) in nums.iter().enumerate() {
        while let Some(&last) = ans.last() {
            if last > x && ans.len() + (n - i) > k {
                ans.pop();
            } else { break; }
        }
        if ans.len() < k { ans.push(x); }
    }
    ans
}
fn main() { println!("{:?}", most_competitive(vec![3,5,2,6], 2)); }
#[cfg(test)]
mod tests {
    use super::most_competitive;
    #[test]
    fn example_one() { assert_eq!(most_competitive(vec![3,5,2,6], 2), vec![2,6]); }
}''',

1674: r'''fn min_moves(nums: Vec<i32>) -> i32 {
    let mut v: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
    v.sort_unstable();
    let med = v[v.len() / 2];
    v.iter().map(|&x| (x - med).abs()).sum::<i64>() as i32
}
fn main() { println!("{}", min_moves(vec![1,2,3])); }
#[cfg(test)]
mod tests {
    use super::min_moves;
    #[test]
    fn example_one() { assert_eq!(min_moves(vec![1,2,3]), 2); }
}''',

1675: r'''use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn minimum_deviation(nums: Vec<i32>) -> i32 {
    let mut max_heap = BinaryHeap::new();
    let mut min_heap = BinaryHeap::new();
    let mut mn = i32::MAX;
    for x in nums {
        let v = if x % 2 == 1 { x * 2 } else { x };
        mn = mn.min(v);
        max_heap.push(v);
        min_heap.push(Reverse(v));
    }
    let mut ans = i32::MAX;
    while max_heap.peek().copied().unwrap_or(0) > mn {
        let mx = max_heap.pop().unwrap();
        ans = ans.min(mx - mn);
        let half = mx / 2;
        max_heap.push(half);
        min_heap.push(Reverse(half));
        mn = min_heap.peek().map(|Reverse(v)| *v).unwrap_or(mn).min(mn);
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

1676: r'''use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

fn lowest_common_ancestor(root: &Option<Box<TreeNode>>, nodes: Vec<Option<Box<TreeNode>>>) -> Option<Box<TreeNode>> {
    let targets: HashSet<i32> = nodes.iter().filter_map(|n| n.as_ref().map(|x| x.val)).collect();
    fn dfs(r: &Option<Box<TreeNode>>, t: &HashSet<i32>) -> (i32, Option<Box<TreeNode>>) {
        let Some(n) = r else { return (0, None); };
        let (lc, ll) = dfs(&n.left, t);
        let (rc, rl) = dfs(&n.right, t);
        let mut cnt = lc + rc + if t.contains(&n.val) { 1 } else { 0 };
        if ll.is_some() { return (cnt, ll); }
        if rl.is_some() { return (cnt, rl); }
        if cnt == t.len() {
            return (cnt, Some(TreeNode { val: n.val, left: None, right: None }));
        }
        (cnt, None)
    }
    dfs(root, &targets).1
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

1678: r'''fn interpret(command: String) -> String {
    command.replace("()", "o").replace("(al)", "al")
}
fn main() { println!("{}", interpret("G()(al)".into())); }
#[cfg(test)]
mod tests {
    use super::interpret;
    #[test]
    fn example_one() { assert_eq!(interpret("G()(al)".into()), "Goal"); }
    #[test]
    fn example_two() { assert_eq!(interpret("G()()()()(al)".into()), "Gooooal"); }
}''',

1679: r'''use std::collections::HashMap;

fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut cnt = HashMap::new();
    let mut ans = 0i32;
    for x in nums {
        let need = k - x;
        if let Some(c) = cnt.get_mut(&need) {
            if *c > 0 { *c -= 1; ans += 1; continue; }
        }
        *cnt.entry(x).or_insert(0) += 1;
    }
    ans
}
fn main() { println!("{}", max_operations(vec![1,2,3,4], 5)); }
#[cfg(test)]
mod tests {
    use super::max_operations;
    #[test]
    fn example_one() { assert_eq!(max_operations(vec![1,2,3,4], 5), 2); }
}''',

1680: r'''fn concatenated_binary(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut ans = 0i64;
    for i in 1..=n as i64 {
        let bits = (64 - (i ^ (i - 1)).leading_zeros()) as i32 - 1;
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
    let mut cnt = [0i32; 21];
    for &x in &nums { cnt[x as usize] += 1; }
    let mut dp = vec![vec![i32::MAX; 1 << n]; k + 1];
    dp[0][0] = 0;
    for mask in 0..(1 << n) {
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
        for prev in 0..mask {
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
    let mut dp = vec![vec![0i32; n]; n];
    for i in (0..n).rev() {
        for j in i..n {
            if i == j { dp[i][j] = 1; continue; }
            if s[i] == s[j] && (j - i < 2 || dp[i + 1][j - 1] > 0) {
                dp[i][j] = if j - i >= 2 { dp[i + 1][j - 1] + 2 } else { 2 };
            }
            dp[i][j] = dp[i][j].max(dp[i + 1][j]).max(dp[i][j - 1]);
        }
    }
    let mut ans = 0i32;
    for i in 0..n {
        for j in i..n {
            let len = dp[i][j];
            if len >= k { ans = ans.max(len); }
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

1684: r'''fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let mut mask = 0u32;
    for c in allowed.bytes() { mask |= 1 << (c - b'a'); }
    words.iter().filter(|w| w.bytes().all(|c| (mask >> (c - b'a')) & 1 == 1)).count() as i32
}
fn main() { println!("{}", count_consistent_strings("ab".into(), vec!["ad".into(),"bd".into(),"aaab".into(),"baa".into(),"badab".into()])); }
#[cfg(test)]
mod tests {
    use super::count_consistent_strings;
    #[test]
    fn example_one() { assert_eq!(count_consistent_strings("ab".into(), vec!["ad".into(),"bd".into(),"aaab".into(),"baa".into(),"badab".into()]), 2); }
}''',

1685: r'''fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let total: i64 = nums.iter().map(|&x| x as i64).sum();
    let mut pref = 0i64;
    let mut ans = vec![0i32; n];
    for i in 0..n {
        let left = pref;
        let right = total - pref - nums[i] as i64;
        ans[i] = (nums[i] as i64 * i as i64 - left + right - nums[i] as i64 * (n - i - 1) as i64) as i32;
        pref += nums[i] as i64;
    }
    ans
}
fn main() { println!("{:?}", get_sum_absolute_differences(vec![2,3,5])); }
#[cfg(test)]
mod tests {
    use super::get_sum_absolute_differences;
    #[test]
    fn example_one() { assert_eq!(get_sum_absolute_differences(vec![2,3,5]), vec![4,3,5]); }
}''',

1686: r'''fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
    let mut v: Vec<(i32, i32)> = alice_values.into_iter().zip(bob_values).map(|(a, b)| (a + b, a - b)).collect();
    v.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    let mut a = 0i64;
    let mut b = 0i64;
    for (i, &(sum, diff)) in v.iter().enumerate() {
        if i % 2 == 0 { a += (sum + diff) / 2; } else { b += (sum - diff) / 2; }
    }
    if a == b { 0 } else if a > b { 1 } else { -1 }
}
fn main() { println!("{}", stone_game_vi(vec![1,3], vec![2,4])); }
#[cfg(test)]
mod tests {
    use super::stone_game_vi;
    #[test]
    fn example_one() { assert_eq!(stone_game_vi(vec![1,3], vec![2,4]), 1); }
}''',

1687: r'''fn box_delivering(boxes: Vec<Vec<i32>>, ports_count: i32, max_boxes: i32, max_weight: i32) -> i32 {
    let n = boxes.len();
    let mut dp = vec![i32::MAX; n + 1];
    dp[0] = 0;
    let mut w = 0i64;
    let mut cnt = 0i32;
    let mut l = 0usize;
    for r in 0..n {
        w += boxes[r][2] as i64;
        cnt += 1;
        if r > 0 && boxes[r][0] != boxes[r - 1][1] { /* port change counted in trip formula */ }
        while l <= r && (cnt > max_boxes || w > max_weight as i64) {
            w -= boxes[l][2] as i64;
            cnt -= 1;
            l += 1;
        }
        let mut cost = dp[l] + (r - l + 1) as i32;
        if l > 0 { cost += 1; }
        if boxes[l][0] != boxes[r][1] { cost += 1; }
        dp[r + 1] = dp[r + 1].min(cost);
        for i in l..r {
            if boxes[i][1] != boxes[i + 1][0] {
                let c = dp[i + 1] + (r - i) as i32 + if i + 1 > 0 { 1 } else { 0 } + 1;
                dp[r + 1] = dp[r + 1].min(c);
            }
        }
    }
    dp[n]
}
fn main() { println!("{}", box_delivering(vec![vec![2,3,3],vec![3,3,3],vec![3,3,3],vec![2,3,3]], 3, 3, 10)); }
#[cfg(test)]
mod tests {
    use super::box_delivering;
    #[test]
    fn example_one() { assert_eq!(box_delivering(vec![vec![2,3,3],vec![3,3,3],vec![3,3,3],vec![2,3,3]], 3, 3, 10), 6); }
}''',

1688: r'''fn number_of_matches(n: i32) -> i32 {
    n - 1
}
fn main() { println!("{}", number_of_matches(7)); }
#[cfg(test)]
mod tests {
    use super::number_of_matches;
    #[test]
    fn example_one() { assert_eq!(number_of_matches(7), 6); }
}''',

1689: r'''fn max_distance(arr: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0i32;
    let mut col_min = vec![i32::MAX; arr[0].len()];
    let mut col_max = vec![i32::MIN; arr[0].len()];
    for row in &arr {
        for (j, &v) in row.iter().enumerate() {
            col_min[j] = col_min[j].min(v);
            col_max[j] = col_max[j].max(v);
        }
    }
    for j in 0..arr[0].len() {
        ans = ans.max(col_max[j] - col_min[j]);
    }
    ans
}
fn main() { println!("{}", max_distance(vec![vec![8,7],vec![5,2]])); }
#[cfg(test)]
mod tests {
    use super::max_distance;
    #[test]
    fn example_one() { assert_eq!(max_distance(vec![vec![8,7],vec![5,2]]), 6); }
}''',

1690: r'''fn stone_game_vii(stones: Vec<i32>) -> i32 {
    let n = stones.len();
    let mut pref = vec![0i32; n + 1];
    for i in 0..n { pref[i + 1] = pref[i] + stones[i]; }
    let mut dp = vec![vec![0i32; n]; n];
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            let sum = pref[j + 1] - pref[i];
            dp[i][j] = (sum - stones[i] - dp[i + 1][j]).max(sum - stones[j] - dp[i][j - 1]);
        }
    }
    dp[0][n - 1]
}
fn main() { println!("{}", stone_game_vii(vec![5,3,1,4,2])); }
#[cfg(test)]
mod tests {
    use super::stone_game_vii;
    #[test]
    fn example_one() { assert_eq!(stone_game_vii(vec![5,3,1,4,2]), 6); }
}''',

1691: r'''fn max_height(cuboids: Vec<Vec<i32>>) -> i32 {
    let mut v: Vec<[i32; 3]> = cuboids.iter().map(|c| {
        let mut a = [c[0], c[1], c[2]];
        a.sort_unstable();
        a
    }).collect();
    v.sort_unstable();
    let n = v.len();
    let mut dp = vec![0i32; n];
    for i in 0..n {
        dp[i] = v[i][2];
        for j in 0..i {
            if v[j][0] <= v[i][0] && v[j][1] <= v[i][1] && v[j][2] <= v[i][2] {
                dp[i] = dp[i].max(dp[j] + v[i][2]);
            }
        }
    }
    *dp.iter().max().unwrap()
}
fn main() { println!("{}", max_height(vec![vec![2,1,2],vec![3,1,2]])); }
#[cfg(test)]
mod tests {
    use super::max_height;
    #[test]
    fn example_one() { assert_eq!(max_height(vec![vec![2,1,2],vec![3,1,2]]), 5); }
}''',

1692: r'''const MOD: i64 = 1_000_000_007;

fn ways_to_distribute(n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let mut dp = vec![vec![0i64; k + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        for j in 1..=k.min(i) {
            dp[i][j] = (dp[i - 1][j - 1] + j as i64 * dp[i - 1][j]) % MOD;
        }
    }
    dp[n][k] as i32
}
fn main() { println!("{}", ways_to_distribute(3, 2)); }
#[cfg(test)]
mod tests {
    use super::ways_to_distribute;
    #[test]
    fn example_one() { assert_eq!(ways_to_distribute(3, 2), 3); }
}''',

1694: r'''fn reformat_number(number: String) -> String {
    let digits: String = number.chars().filter(|c| c.is_ascii_digit()).collect();
    let n = digits.len();
    let mut ans = String::new();
    let mut i = 0usize;
    let mut block = 3usize;
    while i < n {
        if !ans.is_empty() { ans.push('-'); }
        let take = if block == 3 && (n - i) % 4 == 1 && n - i >= 4 { 3 } else if block == 3 && (n - i) % 4 == 0 { 3 } else if n - i <= 4 { n - i } else { 3 };
        let take = if n - i > 4 && (n - i) % 4 == 1 { 3 } else if n - i <= 3 { n - i } else { 3 };
        let rem = n - i;
        let take = if rem > 4 && rem % 4 == 1 { 3 } else if rem <= 3 { rem } else { 3 };
        ans.push_str(&digits[i..i + take]);
        i += take;
    }
    ans
}
fn main() { println!("{}", reformat_number("1-23-45 6".into())); }
#[cfg(test)]
mod tests {
    use super::reformat_number;
    #[test]
    fn example_one() { assert_eq!(reformat_number("1-23-45 6".into()), "123-456"); }
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
fn main() { println!("{}", maximum_unique_subarray(vec![4,2,4,3,3,2])); }
#[cfg(test)]
mod tests {
    use super::maximum_unique_subarray;
    #[test]
    fn example_one() { assert_eq!(maximum_unique_subarray(vec![4,2,4,3,3,2]), 17); }
}''',

1696: r'''use std::collections::VecDeque;

fn max_result(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let n = nums.len();
    let mut dp = vec![0i32; n];
    dp[0] = nums[0];
    let mut dq = VecDeque::from([0usize]);
    for i in 1..n {
        while dq.front().copied().unwrap_or(0) < i.saturating_sub(k) { dq.pop_front(); }
        dp[i] = nums[i] + dp[dq[0]];
        while dq.back().copied().map(|j| dp[j] <= dp[i]).unwrap_or(false) { dq.pop_back(); }
        dq.push_back(i);
    }
    dp[n - 1]
}
fn main() { println!("{}", max_result(vec![1,-1,-2,4,-7,3], 2)); }
#[cfg(test)]
mod tests {
    use super::max_result;
    #[test]
    fn example_one() { assert_eq!(max_result(vec![1,-1,-2,4,-7,3], 2), 7); }
}''',

1697: r'''struct Dsu {
    p: Vec<usize>,
}
impl Dsu {
    fn new(n: usize) -> Self { Self { p: (0..n).collect() } }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x { self.p[x] = self.find(self.p[x]); }
        self.p[x]
    }
    fn unite(&mut self, a: usize, b: usize) {
        let (a, b) = (self.find(a), self.find(b));
        if a != b { self.p[a] = b; }
    }
}

fn distance_limited_paths_exist(n: i32, edge_list: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = n as usize;
    let mut edges: Vec<(i32, usize, usize)> = edge_list.iter().map(|e| (e[2], e[0] as usize, e[1] as usize)).collect();
    edges.sort_unstable();
    let mut qs: Vec<(i32, usize, usize, usize)> = queries.iter().enumerate().map(|(i, q)| (q[2], q[0] as usize, q[1] as usize, i)).collect();
    qs.sort_unstable();
    let mut dsu = Dsu::new(n);
    let mut ans = vec![false; queries.len()];
    let mut ei = 0usize;
    for (limit, p, q, idx) in qs {
        while ei < edges.len() && edges[ei].0 < limit {
            dsu.unite(edges[ei].1, edges[ei].2);
            ei += 1;
        }
        ans[idx] = dsu.find(p) == dsu.find(q);
    }
    ans
}
fn main() { println!("{:?}", distance_limited_paths_exist(3, vec![vec![0,1,2],vec![1,2,4],vec![2,0,8],vec![1,0,16]], vec![vec![0,1,2],vec![0,2,6]])); }
#[cfg(test)]
mod tests {
    use super::distance_limited_paths_exist;
    #[test]
    fn example_one() {
        assert_eq!(distance_limited_paths_exist(3, vec![vec![0,1,2],vec![1,2,4],vec![2,0,8],vec![1,0,16]], vec![vec![0,1,2],vec![0,2,6]]), vec![false, true]);
    }
}''',

1698: r'''fn count_distinct(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut set = std::collections::HashSet::new();
    for i in 0..n {
        let mut h = 0u64;
        for j in i..n {
            h = h.wrapping_mul(131).wrapping_add(b[j] as u64);
            set.insert(h);
        }
    }
    set.len() as i32
}
fn main() { println!("{}", count_distinct("aabbaba".into())); }
#[cfg(test)]
mod tests {
    use super::count_distinct;
    #[test]
    fn example_one() { assert_eq!(count_distinct("aabbaba".into()), 21); }
}''',

1700: r'''use std::collections::VecDeque;

fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut cnt = [0i32; 2];
    for s in students { cnt[s as usize] += 1; }
    for &sw in &sandwiches {
        if cnt[sw as usize] == 0 { return cnt[0] + cnt[1]; }
        cnt[sw as usize] -= 1;
    }
    0
}
fn main() { println!("{}", count_students(vec![1,1,0,0], vec![0,1,0,1])); }
#[cfg(test)]
mod tests {
    use super::count_students;
    #[test]
    fn example_one() { assert_eq!(count_students(vec![1,1,0,0], vec![0,1,0,1]), 0); }
}''',

1701: r'''fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let mut cur = 0i64;
    let mut total = 0i64;
    for c in customers {
        cur = cur.max(c[0] as i64) + c[1] as i64;
        total += cur - c[0] as i64;
    }
    total as f64 / customers.len() as f64
}
fn main() { println!("{}", average_waiting_time(vec![vec![1,2],vec![2,5],vec![4,3]])); }
#[cfg(test)]
mod tests {
    use super::average_waiting_time;
    #[test]
    fn example_one() { assert!((average_waiting_time(vec![vec![1,2],vec![2,5],vec![4,3]]) - 5.0).abs() < 1e-5); }
}''',

1702: r'''fn maximum_binary_string(binary: String) -> String {
    let mut ones = 0usize;
    let mut zeros_after = 0usize;
    let mut seen_zero = false;
    for c in binary.bytes() {
        if c == b'1' {
            if seen_zero { ones += 1; }
        } else {
            seen_zero = true;
            zeros_after += 1;
        }
    }
    if zeros_after == 0 { return binary; }
    let mut ans = String::new();
    ans.push_str(&"0".repeat(ones));
    ans.push('0');
    ans.push_str(&"1".repeat(zeros_after - 1 + binary.len() - ones - zeros_after));
    ans
}
fn main() { println!("{}", maximum_binary_string("000110".into())); }
#[cfg(test)]
mod tests {
    use super::maximum_binary_string;
    #[test]
    fn example_one() { assert_eq!(maximum_binary_string("000110".into()), "111011"); }
}''',

1703: r'''fn min_swaps(nums: Vec<i32>, k: i32) -> i32 {
    let mut ones = vec![];
    for (i, &x) in nums.iter().enumerate() {
        if x == 1 { ones.push(i as i32); }
    }
    if ones.len() < k as usize { return -1; }
    let k = k as usize;
    let mut best = i32::MAX;
    let mut cur = 0i32;
    for i in 0..k {
        cur += ones[i] - ones[0] - i as i32;
    }
    best = best.min(cur);
    for i in 1..=ones.len() - k {
        cur -= ones[i + k - 1] - ones[i - 1] - (k as i32 - 1);
        cur += ones[i + k - 1] - ones[i] - (k as i32 - 1);
        let nc = (0..k).map(|j| (ones[i + j] - (ones[i] + j as i32)).abs()).sum::<i32>();
        let mut nc = 0i32;
        for j in 0..k {
            nc += ones[i + j] - (ones[i - 1] + 1 + j as i32);
        }
        best = best.min(nc);
    }
    best
}
fn main() { println!("{}", min_swaps(vec![1,0,0,1,0,1], 2)); }
#[cfg(test)]
mod tests {
    use super::min_swaps;
    #[test]
    fn example_one() { assert_eq!(min_swaps(vec![1,0,0,1,0,1], 2), 1); }
}''',

1704: r'''fn halves_are_alike(s: String) -> bool {
    let vowels: std::collections::HashSet<u8> = "aeiouAEIOU".bytes().collect();
    let b = s.as_bytes();
    let n = b.len() / 2;
    let a: i32 = b[..n].iter().filter(|&&c| vowels.contains(&c)).count() as i32;
    let c: i32 = b[n..].iter().filter(|&&c| vowels.contains(&c)).count() as i32;
    a == c
}
fn main() { println!("{}", halves_are_alike("book".into())); }
#[cfg(test)]
mod tests {
    use super::halves_are_alike;
    #[test]
    fn example_one() { assert!(halves_are_alike("book".into())); }
}''',

1705: r'''fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    let mut heap = BinaryHeap::new();
    let mut fresh = 0i32;
    let mut ans = 0i32;
    for day in 0..200000 {
        if day < apples.len() {
            if apples[day] > 0 { heap.push(Reverse((days[day] + day, apples[day]))); }
        }
        while let Some(Reverse((exp, _))) = heap.peek() {
            if *exp <= day { heap.pop(); } else { break; }
        }
        if let Some(Reverse((exp, mut cnt))) = heap.pop() {
            ans += 1;
            cnt -= 1;
            if cnt > 0 { heap.push(Reverse((exp, cnt))); }
        }
        if day >= apples.len() && heap.is_empty() { break; }
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

1706: r'''fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let m = grid.len();
    let n = grid[0].len();
    let mut ans = vec![-1; n];
    for c in 0..n {
        let mut col = c as i32;
        let mut row = 0i32;
        loop {
            if row < 0 || row >= m as i32 { break; }
            let d = grid[row as usize][col as usize];
            let nc = col + d;
            if nc < 0 || nc >= n as i32 || grid[row as usize][nc as usize] != d {
                col = -1;
                break;
            }
            row += 1;
            col = nc;
        }
        if row == m as i32 { ans[c] = col; }
    }
    ans
}
fn main() { println!("{:?}", find_ball(vec![vec![1,1,1,-1,-1],vec![1,1,1,-1,-1],vec![-1,-1,-1,1,1],vec![1,1,1,1,-1],vec![-1,-1,-1,-1,-1]])); }
#[cfg(test)]
mod tests {
    use super::find_ball;
    #[test]
    fn example_one() { assert_eq!(find_ball(vec![vec![1,1,1,-1,-1],vec![1,1,1,-1,-1],vec![-1,-1,-1,1,1],vec![1,1,1,1,-1],vec![-1,-1,-1,-1,-1]]), vec![1,-1,-1,-1,-1]); }
}''',

1707: r'''struct TrieNode {
    child: [Option<Box<TrieNode>>; 2],
    val: i32,
}

fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_unstable();
    let mut qs: Vec<(i32, i32, usize)> = queries.iter().enumerate().map(|(i, q)| (q[1], q[0], i)).collect();
    qs.sort_unstable();
    let mut root = TrieNode { child: [None, None], val: -1 };
    let mut ans = vec![-1; queries.len()];
    let mut ni = 0usize;
    for (limit, x, idx) in qs {
        while ni < nums.len() && nums[ni] <= limit {
            let mut node = &mut root;
            for bit in (0..31).rev() {
                let b = (nums[ni] >> bit) & 1;
                node = node.child[b as usize].get_or_insert_with(|| Box::new(TrieNode { child: [None, None], val: -1 }));
            }
            node.val = nums[ni];
            ni += 1;
        }
        if root.child[0].is_none() && root.child[1].is_none() { continue; }
        let mut node = &root;
        let mut xr = 0i32;
        for bit in (0..31).rev() {
            let b = (x >> bit) & 1;
            let want = 1 - b;
            if node.child[want as usize].is_some() {
                xr |= (want ^ b) << bit;
                node = node.child[want as usize].as_ref().unwrap();
            } else {
                xr |= (b ^ b) << bit;
                node = node.child[b as usize].as_ref().unwrap();
            }
        }
        ans[idx] = xr ^ x;
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
    let mut best = 0usize;
    for i in 0..=nums.len() - k {
        let mn = *nums[i..i + k].iter().min().unwrap();
        let mx = *nums[i..i + k].iter().max().unwrap();
        if mx - mn == (k as i32 - 1) { best = i; break; }
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

1710: r'''fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
    let mut v = box_types;
    v.sort_unstable_by(|a, b| b[1].cmp(&a[1]));
    let mut rem = truck_size;
    let mut ans = 0i32;
    for b in v {
        let take = rem.min(b[0]);
        ans += take * b[1];
        rem -= take;
        if rem == 0 { break; }
    }
    ans
}
fn main() { println!("{}", maximum_units(vec![vec![1,3],vec![2,2],vec![3,1]], 4)); }
#[cfg(test)]
mod tests {
    use super::maximum_units;
    #[test]
    fn example_one() { assert_eq!(maximum_units(vec![vec![1,3],vec![2,2],vec![3,1]], 4), 8); }
}''',

1711: r'''const MOD: i64 = 1_000_000_007;

fn count_pairs(deliciousness: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut cnt = HashMap::new();
    let mut ans = 0i64;
    for x in deliciousness {
        let mut p = 1i32;
        while p <= 1 << 21 {
            let need = p - x;
            ans += *cnt.get(&need).unwrap_or(&0) as i64;
            p <<= 1;
        }
        *cnt.entry(x).or_insert(0i64) += 1;
    }
    (ans % MOD) as i32
}
fn main() { println!("{}", count_pairs(vec![1,3,5,7,9])); }
#[cfg(test)]
mod tests {
    use super::count_pairs;
    #[test]
    fn example_one() { assert_eq!(count_pairs(vec![1,3,5,7,9]), 1); }
}''',

1712: r'''const MOD: i64 = 1_000_000_007;

fn ways_to_split(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut pref = vec![0i64; n + 1];
    for i in 0..n { pref[i + 1] = pref[i] + nums[i] as i64; }
    let mut ans = 0i64;
    for i in 0..n - 2 {
        if pref[i + 1] * 2 > pref[n] { break; }
        let mut j = i + 1;
        while j < n - 1 && pref[j + 1] * 2 <= pref[n] {
            if pref[i + 1] <= pref[j + 1] - pref[i + 1] && pref[j + 1] - pref[i + 1] <= pref[n] - pref[j + 1] {
                ans += 1;
            }
            j += 1;
        }
    }
    ans as i32
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
    let mut need = 0i32;
    for &c in t {
        let p = &pos[(c - b'a') as usize];
        match p.binary_search(&idx) {
            Ok(i) | Err(i) if i < p.len() => { idx = p[i] + 1; }
            _ => { need += 1; idx = 0; if let Ok(i) | Err(i) = p.binary_search(&idx) {
                if i < p.len() { idx = p[i] + 1; } else { return -1; }
            } }
        }
    }
    need
}
fn main() { println!("{}", min_operations("abc".into(), "abcbc".into())); }
#[cfg(test)]
mod tests {
    use super::min_operations;
    #[test]
    fn example_one() { assert_eq!(min_operations("abc".into(), "abcbc".into()), 2); }
}''',

1714: r'''fn maximum_gain(s: String) -> i32 {
    let mut a = 0i64;
    let mut b = 0i64;
    let mut ans = 0i64;
    for c in s.bytes() {
        if c == b'a' {
            if b > 0 { b -= 1; ans += 1; } else { a += 1; }
        } else {
            if a > 0 { a -= 1; ans += 1; } else { b += 1; }
        }
    }
    ans as i32
}
fn main() { println!("{}", maximum_gain("cdbcbbaaabab".into())); }
#[cfg(test)]
mod tests {
    use super::maximum_gain;
    #[test]
    fn example_one() { assert_eq!(maximum_gain("cdbcbbaaabab".into()), 4); }
}''',
}
