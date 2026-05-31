/// LeetCode #1601 - Maximum Number Of Achievable Transfer Requests
fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let m = requests.len();
    let mut ans = 0i32;
    for mask in 0usize..(1usize << m) {
        let mut bal = vec![0i32; n];
        for i in 0..m {
            if mask & (1 << i) == 0 { continue; }
            bal[requests[i][0] as usize] -= 1;
            bal[requests[i][1] as usize] += 1;
        }
        if bal.iter().all(|&x| x == 0) {
            ans = ans.max(mask.count_ones() as i32);
        }
    }
    ans
}
fn main() { println!("{}", maximum_requests(3, vec![vec![0,1],vec![1,2],vec![2,0]])); }
#[cfg(test)]
mod tests {
    use super::maximum_requests;
    #[test]
    fn example_one() { assert_eq!(maximum_requests(3, vec![vec![0,1],vec![1,2],vec![2,0]]), 3); }
    #[test]
    fn example_two() { assert_eq!(maximum_requests(4, vec![vec![0,1],vec![1,2],vec![2,3],vec![0,3]]), 0); }
}