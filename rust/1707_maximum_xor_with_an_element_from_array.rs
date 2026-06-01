/// LeetCode #1707 - Maximum Xor With An Element From Array
struct TrieNode {
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
}