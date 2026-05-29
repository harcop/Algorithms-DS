/// LeetCode #1534 - Count Good Triplets
fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
    let n = arr.len();
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if (arr[i] - arr[j]).abs() <= a && (arr[j] - arr[k]).abs() <= b && (arr[i] - arr[k]).abs() <= c {
                    ans += 1;
                }
            }
        }
    }
    ans
}

fn main() {
    println!("{}", count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3));
}

#[cfg(test)]
mod tests {
    use super::count_good_triplets;

    #[test]
    fn example_one() {
        assert_eq!(count_good_triplets(vec![3, 0, 1, 1, 9, 7], 7, 2, 3), 4);
    }
}
