/// LeetCode #1591 - Strange Printer Ii
fn is_printable(grid: Vec<Vec<i32>>) -> bool {
    let n = grid.len();
    let m = grid[0].len();
    let mut lo = vec![vec![i32::MAX; 61]; 61];
    let mut hi = vec![vec![i32::MIN; 61]; 61];
    for i in 0..n {
        for j in 0..m {
            let c = grid[i][j] as usize;
            lo[c][0] = lo[c][0].min(i as i32);
            lo[c][1] = lo[c][1].min(j as i32);
            hi[c][0] = hi[c][0].max(i as i32);
            hi[c][1] = hi[c][1].max(j as i32);
        }
    }
    let mut seen = vec![false; 61];
    let mut order = vec![];
    fn dfs(c: usize, lo: &[Vec<i32>], hi: &[Vec<i32>], seen: &mut [bool], order: &mut Vec<usize>) -> bool {
        if seen[c] { return true; }
        seen[c] = true;
        for d in 1..61 {
            if lo[d][0] == i32::MAX { continue; }
            if lo[d][0] <= hi[c][0] && hi[d][0] >= lo[c][0] && lo[d][1] <= hi[c][1] && hi[d][1] >= lo[c][1]
                && (lo[d][0] < lo[c][0] || hi[d][0] > hi[c][0] || lo[d][1] < lo[c][1] || hi[d][1] > hi[c][1])
            {
                if !dfs(d, lo, hi, seen, order) { return false; }
            }
        }
        order.push(c);
        true
    }
    for c in 1..61 {
        if lo[c][0] != i32::MAX && !dfs(c, &lo, &hi, &mut seen, &mut order) { return false; }
    }
    let mut g = grid.clone();
    for &c in order.iter().rev() {
        for i in lo[c][0] as usize..=hi[c][0] as usize {
            for j in lo[c][1] as usize..=hi[c][1] as usize {
                if g[i][j] != c as i32 { return false; }
                g[i][j] = 0;
            }
        }
    }
    true
}
fn main() { println!("{}", is_printable(vec![vec![1,1,1],vec![1,1,1],vec![1,1,1]])); }
#[cfg(test)]
mod tests {
    use super::is_printable;
    #[test]
    fn example_one() { assert!(is_printable(vec![vec![1,1,1],vec![1,1,1],vec![1,1,1]])); }
}