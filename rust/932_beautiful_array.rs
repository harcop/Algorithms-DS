/// LeetCode #932 - Beautiful Array
fn beautiful_array(n: i32) -> Vec<i32> {
    let mut res = vec![1i32];
    while res.len() < n as usize {
        let mut odd = vec![];
        let mut even = vec![];
        for &x in &res {
            odd.push(2 * x - 1);
            even.push(2 * x);
        }
        odd.extend(even);
        res = odd;
    }
    res
}

fn main() {
    println!("{:?}", beautiful_array(4));
}

#[cfg(test)]
mod tests {
    use super::beautiful_array;

    #[test]
    fn example_one() {
        let a = beautiful_array(4);
        assert_eq!(a.len(), 4);
        for i in 0..a.len() {
            for j in i + 1..a.len() {
                for k in j + 1..a.len() {
                    assert!(a[i] + a[k] != 2 * a[j]);
                }
            }
        }
    }
}
