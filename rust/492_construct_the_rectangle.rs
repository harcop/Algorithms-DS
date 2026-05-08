/// LeetCode #492 - Construct the Rectangle
fn construct_rectangle(area: i32) -> Vec<i32> {
    let mut w = (area as f64).sqrt() as i32;
    while area % w != 0 {
        w -= 1;
    }
    vec![area / w, w]
}

fn main() {
    println!("{:?}", construct_rectangle(4));
}

#[cfg(test)]
mod tests {
    use super::construct_rectangle;

    #[test]
    fn example_one() {
        assert_eq!(construct_rectangle(4), vec![2, 2]);
    }

    #[test]
    fn example_two() {
        assert_eq!(construct_rectangle(37), vec![37, 1]);
    }
}
