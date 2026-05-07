/// LeetCode #440 - K-th Smallest in Lexicographical Order
fn count_steps(mut curr: i64, n: i64) -> i64 {
    let mut steps = 0i64;
    let mut first = curr;
    let mut last = curr + 1;
    while first <= n {
        steps += (n + 1).min(last) - first;
        first *= 10;
        last *= 10;
    }
    steps
}

fn find_kth_number(n: i32, k: i32) -> i32 {
    let mut curr = 1i64;
    let mut k = k as i64 - 1;
    let n = n as i64;
    while k > 0 {
        let steps = count_steps(curr, n);
        if steps <= k {
            k -= steps;
            curr += 1;
        } else {
            curr *= 10;
            k -= 1;
        }
    }
    curr as i32
}

fn main() {
    println!("{}", find_kth_number(13, 2));
}

#[cfg(test)]
mod tests {
    use super::find_kth_number;

    #[test]
    fn example_one() {
        assert_eq!(find_kth_number(13, 2), 10);
    }
}
