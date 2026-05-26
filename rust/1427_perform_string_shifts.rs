/// LeetCode #1427 - Perform String Shifts
fn string_shifts(s: String, shift: Vec<Vec<i32>>) -> String {
    let n = s.len();
    if n == 0 {
        return s;
    }
    let mut x = 0i32;
    for sh in shift {
        if sh[0] == 0 {
            x -= sh[1];
        } else {
            x += sh[1];
        }
    }
    let x = ((x % n as i32) + n as i32) as usize % n;
    format!("{}{}", &s[n - x..], &s[..n - x])
}

fn main() {
    println!(
        "{}",
        string_shifts("abcdefg".into(), vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::string_shifts;

    #[test]
    fn example_one() {
        assert_eq!(string_shifts("abc".into(), vec![vec![0, 1], vec![1, 2]]), "cab");
    }

    #[test]
    fn example_two() {
        assert_eq!(
            string_shifts(
                "abcdefg".into(),
                vec![vec![1, 1], vec![1, 1], vec![0, 2], vec![1, 3]],
            ),
            "efgabcd"
        );
    }
}
