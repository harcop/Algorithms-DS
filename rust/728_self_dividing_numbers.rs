/// LeetCode #728 - Self Dividing Numbers
fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut out = vec![];
    for x in left..=right {
        let mut v = x;
        let mut ok = true;
        if v == 0 {
            continue;
        }
        while v > 0 {
            let d = v % 10;
            if d == 0 || x % d != 0 {
                ok = false;
                break;
            }
            v /= 10;
        }
        if ok {
            out.push(x);
        }
    }
    out
}

fn main() {
    println!("{:?}", self_dividing_numbers(1, 22));
}

#[cfg(test)]
mod tests {
    use super::self_dividing_numbers;

    #[test]
    fn example_one() {
        assert_eq!(
            self_dividing_numbers(1, 22),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        );
    }
}
