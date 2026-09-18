/// LeetCode #3800 - Minimum Cost to Make Two Binary Strings Equal
fn minimum_cost(s: String, t: String, flip_cost: i32, swap_cost: i32, cross_cost: i32) -> i64 {
    let mut diff = [0i64; 2];
    for (c1, c2) in s.bytes().zip(t.bytes()) {
        if c1 != c2 {
            diff[(c1 - b'0') as usize] += 1;
        }
    }
    let flip = flip_cost as i64;
    let sw = swap_cost as i64;
    let cr = cross_cost as i64;
    let mut ans = (diff[0] + diff[1]) * flip;
    let mx = diff[0].max(diff[1]);
    let mn = diff[0].min(diff[1]);
    ans = ans.min(mn * sw + (mx - mn) * flip);
    let avg = (mx + mn) / 2;
    ans = ans.min((avg - mn) * cr + avg * sw + (mx + mn - avg * 2) * flip);
    ans
}

fn main() {
    println!(
        "{}",
        minimum_cost("01000".into(), "10111".into(), 10, 2, 2)
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_cost("01000".into(), "10111".into(), 10, 2, 2),
            16
        );
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_cost("001".into(), "110".into(), 2, 100, 100), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_cost("1010".into(), "1010".into(), 5, 5, 5), 0);
    }
}
