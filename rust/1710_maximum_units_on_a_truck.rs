/// LeetCode #1710 - Maximum Units On A Truck
fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
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
}