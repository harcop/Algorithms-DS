/// LeetCode #1088 - Confusing Number II
fn confusing_number_ii(n: i32) -> i32 {
    let digits = [0, 1, 6, 8, 9];
    let mut count = 0;
    dfs(0, 0, 1, n as i64, &digits, &mut count);
    count
}

fn dfs(num: i64, rotated: i64, base: i64, n: i64, digits: &[i64], count: &mut i32) {
    if num != rotated {
        *count += 1;
    }
    for &d in digits {
        let next = num * 10 + d;
        if next == 0 || next > n {
            continue;
        }
        let r = match d {
            0 => 0,
            1 => 1,
            6 => 9,
            8 => 8,
            9 => 6,
            _ => continue,
        };
        dfs(next, r * base + rotated, base * 10, n, digits, count);
    }
}

fn main() {
    println!("{}", confusing_number_ii(20));
}

#[cfg(test)]
mod tests {
    use super::confusing_number_ii;

    #[test]
    fn example_one() {
        assert_eq!(confusing_number_ii(20), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(confusing_number_ii(100), 19);
    }
}
