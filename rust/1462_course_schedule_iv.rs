/// LeetCode #1462 - Course Schedule Iv
fn check_if_valid(n: i32, queries: Vec<Vec<i32>>, prerequisites: Vec<Vec<i32>>) -> Vec<bool> {
    let n = n as usize;
    let mut reach = vec![vec![false; n]; n];
    for p in &prerequisites {
        reach[p[0] as usize][p[1] as usize] = true;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                reach[i][j] = reach[i][j] || (reach[i][k] && reach[k][j]);
            }
        }
    }
    queries.iter().map(|q| reach[q[0] as usize][q[1] as usize]).collect()
}
fn main() { println!("{:?}", check_if_valid(2, vec![vec![1,0],vec![1,1]], vec![vec![1,0]])); }
#[cfg(test)]
mod tests {
    use super::check_if_valid;
    #[test]
    fn example_one() { assert_eq!(check_if_valid(2, vec![vec![1,0],vec![1,1]], vec![vec![1,0]]), vec![true,false]); }
    #[test]
    fn example_two() { assert_eq!(check_if_valid(2, vec![vec![1,0],vec![1,1]], vec![]), vec![false,false]); }
}