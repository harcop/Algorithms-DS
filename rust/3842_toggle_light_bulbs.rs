/// LeetCode #3842 - Toggle Light Bulbs
fn toggle_light_bulbs(bulbs: Vec<i32>) -> Vec<i32> {
    let mut st = [0; 101];
    for x in bulbs {
        st[x as usize] ^= 1;
    }
    (1..101).filter(|&i| st[i] == 1).map(|i| i as i32).collect()
}

fn main() {
    println!("{:?}", toggle_light_bulbs(vec![10, 30, 20, 10]));
}

#[cfg(test)]
mod tests {
    use super::toggle_light_bulbs;

    #[test]
    fn example1() {
        assert_eq!(toggle_light_bulbs(vec![10, 30, 20, 10]), vec![20, 30]);
    }

    #[test]
    fn example2() {
        assert_eq!(toggle_light_bulbs(vec![100, 100]), Vec::<i32>::new());
    }
}
