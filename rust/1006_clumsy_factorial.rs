/// LeetCode #1006 - Clumsy Factorial
fn clumsy(n: i32) -> i32 {
    if n <= 2 {
        return n;
    }
    if n == 3 {
        return 6;
    }
    let mut stack = vec![n, n - 1];
    let mut cur = n - 2;
    let mut op = 0usize;
    while cur >= 1 {
        match op % 4 {
            0 => {
                let a = stack.pop().unwrap();
                stack.push(a * cur);
            }
            1 => {
                let a = stack.pop().unwrap();
                stack.push(a / cur);
            }
            2 => stack.push(cur),
            _ => {
                stack.push(cur);
                op += 1;
                cur -= 1;
                continue;
            }
        }
        op += 1;
        cur -= 1;
    }
    let mut sum = stack[0];
    for &x in &stack[1..] {
        sum += x;
    }
    sum
}

fn main() {
    println!("{}", clumsy(4));
}

#[cfg(test)]
mod tests {
    use super::clumsy;

    #[test]
    fn example_one() {
        assert_eq!(clumsy(4), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(clumsy(10), 12);
    }
}
