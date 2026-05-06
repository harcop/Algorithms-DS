/// LeetCode #401 - Binary Watch
fn read_binary_watch(turned_on: i32) -> Vec<String> {
    let mut out = vec![];
    for h in 0..12 {
        for m in 0..60 {
            let bits = ((h as u32).count_ones() + (m as u32).count_ones()) as i32;
            if bits == turned_on {
                out.push(format!("{}:{:02}", h, m));
            }
        }
    }
    out
}

fn main() {
    println!("{}", read_binary_watch(1).len());
}

#[cfg(test)]
mod tests {
    use super::read_binary_watch;

    #[test]
    fn example_one() {
        let v = read_binary_watch(1);
        assert!(v.contains(&"0:01".to_string()));
    }
}
