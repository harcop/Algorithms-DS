/// LeetCode #753 - Cracking the Safe
use std::collections::HashSet;

fn crack_safe(n: i32, k: i32) -> String {
    let n = n as usize;
    let k = k as usize;
    let kn = k.pow(n as u32);
    let mut seen = HashSet::new();
    let mut path: Vec<usize> = Vec::with_capacity(kn);

    fn dfs(node: usize, k: usize, kn: usize, seen: &mut HashSet<usize>, path: &mut Vec<usize>) {
        for d in (0..k).rev() {
            let nei = node * k + d;
            if seen.insert(nei) {
                dfs(nei % kn, k, kn, seen, path);
                path.push(d);
            }
        }
    }

    dfs(0, k, kn, &mut seen, &mut path);
    let prefix = "0".repeat(n.saturating_sub(1));
    let mut s = prefix;
    for d in path {
        s.push((b'0' + d as u8) as char);
    }
    s
}

fn main() {
    println!("{}", crack_safe(1, 2));
}

#[cfg(test)]
mod tests {
    use super::crack_safe;

    #[test]
    fn example_one() {
        let s = crack_safe(1, 2);
        assert!(s.contains('0') && s.contains('1'));
    }
}
