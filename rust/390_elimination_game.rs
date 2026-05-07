/// LeetCode #390 - Elimination Game (every other from alternating ends)
fn last_remaining(n: i32) -> i32 {
    let mut head = 1i32;
    let mut step = 1i32;
    let mut rem = n;
    let mut left = true;
    while rem > 1 {
        if left || rem % 2 == 1 {
            head += step;
        }
        rem /= 2;
        step *= 2;
        left = !left;
    }
    head
}

fn main() {
    println!("{}", last_remaining(9));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        assert_eq!(last_remaining(9), 6);
    }
}
