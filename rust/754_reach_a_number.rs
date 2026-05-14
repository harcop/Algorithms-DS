/// LeetCode #754 - Reach a Number
fn reach_number(target: i32) -> i32 {
    let t = target.abs();
    let mut k = 0i32;
    let mut s = 0i32;
    while s < t || (s - t) % 2 != 0 {
        k += 1;
        s += k;
    }
    k
}

fn main() {
    println!("{}", reach_number(3));
}

#[cfg(test)]
mod tests {
    use super::reach_number;

    #[test]
    fn example_one() {
        assert_eq!(reach_number(3), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(reach_number(2), 3);
    }
}
