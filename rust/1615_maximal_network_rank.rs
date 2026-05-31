/// LeetCode #1615 - Maximal Network Rank
fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut deg = vec![0i32; n];
    for r in &roads {
        deg[r[0] as usize] += 1;
        deg[r[1] as usize] += 1;
    }
    let mut ans = 0i32;
    for i in 0..n {
        for j in i + 1..n {
            let mut s = deg[i] + deg[j];
            if roads.iter().any(|r| (r[0] as usize == i && r[1] as usize == j) || (r[0] as usize == j && r[1] as usize == i)) {
                s -= 1;
            }
            ans = ans.max(s);
        }
    }
    ans
}
fn main() { println!("{}", maximal_network_rank(4, vec![vec![0,1],vec![0,3],vec![1,2],vec![1,3]])); }
#[cfg(test)]
mod tests {
    use super::maximal_network_rank;
    #[test]
    fn example_one() { assert_eq!(maximal_network_rank(4, vec![vec![0,1],vec![0,3],vec![1,2],vec![1,3]]), 4); }
}