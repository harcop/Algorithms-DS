/// LeetCode #3015 - Count the Number of Houses at a Certain Distance I
fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i32> {
    let n = n as usize;
    let x = (x - 1) as usize;
    let y = (y - 1) as usize;
    let mut ans = vec![0i32; n];

    for i in 0..n {
        for j in (i + 1)..n {
            let a = j - i;
            let b = (i as i32 - x as i32).unsigned_abs() as usize
                + 1
                + (j as i32 - y as i32).unsigned_abs() as usize;
            let c = (i as i32 - y as i32).unsigned_abs() as usize
                + 1
                + (j as i32 - x as i32).unsigned_abs() as usize;
            let d = a.min(b).min(c);
            ans[d - 1] += 2;
        }
    }

    ans
}

fn main() {
    println!("{:?}", count_of_pairs(3, 1, 3));
    println!("{:?}", count_of_pairs(5, 2, 4));
    println!("{:?}", count_of_pairs(4, 1, 1));
}

#[cfg(test)]
mod tests {
    use super::count_of_pairs;

    #[test]
    fn example_one() {
        assert_eq!(count_of_pairs(3, 1, 3), vec![6, 0, 0]);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_of_pairs(5, 2, 4), vec![10, 8, 2, 0, 0]);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_of_pairs(4, 1, 1), vec![6, 4, 2, 0]);
    }
}
