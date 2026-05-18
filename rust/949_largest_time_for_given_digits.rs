/// LeetCode #949 - Largest Time for Given Digits

fn largest_time_from_digits(arr: Vec<i32>) -> String {
    let mut best = -1i32;
    let n = arr.len();
    for i in 0..n {
        for j in 0..n {
            if j == i {
                continue;
            }
            for k in 0..n {
                if k == i || k == j {
                    continue;
                }
                for l in 0..n {
                    if l == i || l == j || l == k {
                        continue;
                    }
                    let h = arr[i] * 10 + arr[j];
                    let m = arr[k] * 10 + arr[l];
                    if h < 24 && m < 60 {
                        best = best.max(h * 60 + m);
                    }
                }
            }
        }
    }
    if best < 0 {
        return String::new();
    }
    format!("{:02}:{:02}", best / 60, best % 60)
}

fn main() {
    println!("{}", largest_time_from_digits(vec![1, 2, 3, 4]));
}

#[cfg(test)]
mod tests {
    use super::largest_time_from_digits;

    #[test]
    fn example_one() {
        assert_eq!(largest_time_from_digits(vec![1, 2, 3, 4]), "23:41");
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_time_from_digits(vec![5, 5, 5, 5]), "");
    }
}
