/// LeetCode #2197 - Replace Non-Coprime Numbers in Array
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
    let mut stk = Vec::new();
    for x in nums {
        stk.push(x as i64);
        while stk.len() > 1 {
            let x = *stk.last().unwrap();
            let y = stk[stk.len() - 2];
            let g = gcd(x, y);
            if g == 1 {
                break;
            }
            stk.pop();
            let last = stk.last_mut().unwrap();
            *last = x / g * y;
        }
    }
    stk.into_iter().map(|v| v as i32).collect()
}

fn main() {
    println!("{:?}", replace_non_coprimes(vec![6, 4, 3, 2, 7, 6, 2]));
}

#[cfg(test)]
mod tests {
    use super::replace_non_coprimes;

    #[test]
    fn example_one() {
        assert_eq!(
            replace_non_coprimes(vec![6, 4, 3, 2, 7, 6, 2]),
            vec![12, 7, 6]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            replace_non_coprimes(vec![2, 2, 1, 1, 3, 3, 3]),
            vec![2, 1, 1, 3]
        );
    }
}
