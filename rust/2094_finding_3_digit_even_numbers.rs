/// LeetCode #2094 - Finding 3-Digit Even Numbers
fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
    let mut cnt = [0i32; 10];
    for d in digits {
        cnt[d as usize] += 1;
    }

    let mut ans = Vec::new();
    for x in (100..=998).step_by(2) {
        let mut need = [0i32; 10];
        let mut y = x;
        for _ in 0..3 {
            need[y % 10] += 1;
            y /= 10;
        }
        if (0..10).all(|i| need[i] <= cnt[i]) {
            ans.push(x as i32);
        }
    }
    ans
}

fn main() {
    println!("{:?}", find_even_numbers(vec![2, 1, 3, 0]));
}

#[cfg(test)]
mod tests {
    use super::find_even_numbers;

    #[test]
    fn example_one() {
        assert_eq!(
            find_even_numbers(vec![2, 1, 3, 0]),
            vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(find_even_numbers(vec![2, 2, 8, 8, 2]), vec![222, 228, 282, 288, 822, 828, 882]);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_even_numbers(vec![3, 7, 5]), Vec::<i32>::new());
    }
}
