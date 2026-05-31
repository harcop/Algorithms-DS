/// LeetCode #1588 - Sum Of All Odd Length Subarrays
fn sum_odd_length_subarrs(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut ans = 0i32;
    for i in 0..n {
        let mut s = 0i32;
        for j in i..n {
            s += arr[j];
            if (j - i) % 2 == 0 { ans += s; }
        }
    }
    ans
}
fn main() { println!("{}", sum_odd_length_subarrs(vec![1,4,2,5,3])); }
#[cfg(test)]
mod tests {
    use super::sum_odd_length_subarrs;
    #[test]
    fn example_one() { assert_eq!(sum_odd_length_subarrs(vec![1,4,2,5,3]), 58); }
}