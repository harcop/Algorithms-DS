/// LeetCode #3950 - Exactly One Consecutive Set Bits Pair
fn consecutive_set_bits(mut n: i32) -> bool {
    let mut pre = 0;
    let mut vis = false;
    while n > 0 {
        let cur = n & 1;
        if pre == 1 && cur == 1 {
            if vis {
                return false;
            }
            vis = true;
        }
        pre = cur;
        n >>= 1;
    }
    vis
}

fn main() {
    println!("{}", consecutive_set_bits(6));
}

#[cfg(test)]
mod tests {
    use super::consecutive_set_bits;

    #[test]
    fn example1() {
        assert_eq!(consecutive_set_bits(6), true);
    }

    #[test]
    fn example2() {
        assert_eq!(consecutive_set_bits(5), false);
    }
}
