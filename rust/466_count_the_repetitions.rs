/// LeetCode #466 - Count The Repetitions
use std::collections::HashMap;

fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
    let s1b = s1.as_bytes();
    let s2b = s2.as_bytes();
    if s2b.is_empty() {
        return 0;
    }
    let mut idx = 0usize;
    let mut cnt = 0i64;
    let mut repeat = 0i32;
    let mut map: HashMap<usize, (i32, i64)> = HashMap::new();

    while repeat < n1 {
        if let Some(&(prev_r, prev_cnt)) = map.get(&idx) {
            let dr = repeat - prev_r;
            let dc = cnt - prev_cnt;
            let remaining = n1 - repeat;
            if dr > 0 && remaining >= dr {
                let mul = remaining / dr;
                cnt += mul as i64 * dc;
                repeat += mul * dr;
                map.clear();
                continue;
            }
        }
        map.insert(idx, (repeat, cnt));

        for &c in s1b {
            if c == s2b[idx] {
                idx += 1;
                if idx == s2b.len() {
                    cnt += 1;
                    idx = 0;
                }
            }
        }
        repeat += 1;
    }

    (cnt / n2 as i64) as i32
}

fn main() {
    println!(
        "{}",
        get_max_repetitions("acb".into(), 4, "ab".into(), 2)
    );
}

#[cfg(test)]
mod tests {
    use super::get_max_repetitions;

    #[test]
    fn example_one() {
        assert_eq!(get_max_repetitions("acb".into(), 4, "ab".into(), 2), 2);
    }
}
