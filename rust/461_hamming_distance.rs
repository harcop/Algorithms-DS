/// LeetCode #461 - Hamming Distance
fn hamming_distance(mut x: i32, mut y: i32) -> i32 {
    let mut z = x ^ y;
    let mut c = 0;
    while z != 0 {
        c += z & 1;
        z >>= 1;
    }
    c
}

fn main() {
    println!("{}", hamming_distance(1, 4));
}

#[cfg(test)]
mod tests {
    use super::hamming_distance;

    #[test]
    fn example_one() {
        assert_eq!(hamming_distance(1, 4), 2);
    }
}
