/// LeetCode #1122 - Relative Sort Array
fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut cnt = [0i32; 1001];
    for &x in &arr1 {
        cnt[x as usize] += 1;
    }
    let mut ans = Vec::with_capacity(arr1.len());
    for &x in &arr2 {
        for _ in 0..cnt[x as usize] {
            ans.push(x);
        }
        cnt[x as usize] = 0;
    }
    let mut rest: Vec<i32> = (0..1001i32)
        .flat_map(|i| std::iter::repeat_n(i, cnt[i as usize] as usize))
        .collect();
    rest.sort_unstable();
    ans.extend(rest);
    ans
}

fn main() {
    println!("{:?}", relative_sort_array(vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19], vec![2, 1, 4, 3, 9, 6]));
}

#[cfg(test)]
mod tests {
    use super::relative_sort_array;

    #[test]
    fn example_one() {
        assert_eq!(
            relative_sort_array(vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19], vec![2, 1, 4, 3, 9, 6]),
            vec![2, 2, 2, 1, 4, 4, 3, 3, 9, 6, 7, 19]
        );
    }
}
