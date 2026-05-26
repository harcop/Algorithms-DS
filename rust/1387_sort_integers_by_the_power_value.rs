/// LeetCode #1387 - Sort Integers By The Power Value
fn get_power(x: i64) -> i64 {
    let mut steps = 0i64;
    let mut v = x;
    while v != 1 {
        if v % 2 == 0 {
            v /= 2;
        } else {
            v = 3 * v + 1;
        }
        steps += 1;
    }
    steps
}

fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
    let mut vals: Vec<i32> = (lo..=hi).collect();
    vals.sort_by(|&a, &b| {
        let pa = get_power(a as i64);
        let pb = get_power(b as i64);
        pa.cmp(&pb).then_with(|| a.cmp(&b))
    });
    vals[k as usize - 1]
}

fn main() {
    println!("{}", get_kth(12, 15, 2));
}

#[cfg(test)]
mod tests {
    use super::get_kth;

    #[test]
    fn example_one() {
        assert_eq!(get_kth(12, 15, 2), 13);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_kth(1, 1, 1), 1);
    }
}

