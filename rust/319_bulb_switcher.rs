/// LeetCode #319 - Bulb Switcher
fn bulb_switch(n: i32) -> i32 {
    (n as f64).sqrt() as i32
}

fn main() {
    println!("{}", bulb_switch(3));
}

#[cfg(test)]
mod tests {
    use super::bulb_switch;

    #[test]
    fn example_one() {
        assert_eq!(bulb_switch(3), 1);
    }
}
