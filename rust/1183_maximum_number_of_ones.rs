/// LeetCode #1183 - Maximum Number of Ones
fn maximum_number_of_ones(width: i32, height: i32, side_length: i32, max_variety: i32) -> i32 {
    let w = width as i64;
    let h = height as i64;
    let s = side_length as i64;
    let m = max_variety as i64;
    let bw = (w + s - 1) / s;
    let bh = (h + s - 1) / s;
    (bw * bh * m) as i32
}

fn main() {
    println!("{}", maximum_number_of_ones(3, 3, 2, 1));
}

#[cfg(test)]
mod tests {
    use super::maximum_number_of_ones;

    #[test]
    fn example_one() {
        assert_eq!(maximum_number_of_ones(3, 3, 2, 1), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximum_number_of_ones(3, 3, 2, 2), 8);
    }
}
