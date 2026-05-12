/// LeetCode #691 - Stickers to Spell Word
use std::collections::HashMap;

fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
    let target_bytes = target.as_bytes().to_vec();
    let n = target_bytes.len();
    let sticker_counts: Vec<[i32; 26]> = stickers
        .iter()
        .map(|s| {
            let mut c = [0i32; 26];
            for b in s.bytes() {
                c[(b - b'a') as usize] += 1;
            }
            c
        })
        .collect();

    fn dfs(
        mask: u32,
        target: &[u8],
        stickers: &[[i32; 26]],
        memo: &mut HashMap<u32, i32>,
    ) -> i32 {
        if mask == 0 {
            return 0;
        }
        if let Some(&v) = memo.get(&mask) {
            return v;
        }
        let first_bit = mask.trailing_zeros() as usize;
        let first_letter = (target[first_bit] - b'a') as usize;
        let mut best = i32::MAX;
        for stk in stickers {
            if stk[first_letter] == 0 {
                continue;
            }
            let mut new_mask = mask;
            let mut s_copy = *stk;
            for i in 0..target.len() {
                if mask & (1u32 << i) != 0 {
                    let ch = (target[i] - b'a') as usize;
                    if s_copy[ch] > 0 {
                        s_copy[ch] -= 1;
                        new_mask &= !(1u32 << i);
                    }
                }
            }
            if new_mask < mask {
                let sub = dfs(new_mask, target, stickers, memo);
                if sub != i32::MAX {
                    best = best.min(sub + 1);
                }
            }
        }
        memo.insert(mask, best);
        best
    }

    let full = if n == 32 { u32::MAX } else { (1u32 << n) - 1 };
    let mut memo: HashMap<u32, i32> = HashMap::new();
    let r = dfs(full, &target_bytes, &sticker_counts, &mut memo);
    if r == i32::MAX {
        -1
    } else {
        r
    }
}

fn main() {
    println!(
        "{}",
        min_stickers(
            vec![
                "with".into(),
                "example".into(),
                "science".into(),
            ],
            "thehat".into()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_stickers;

    #[test]
    fn example_one() {
        assert_eq!(
            min_stickers(
                vec!["with".into(), "example".into(), "science".into()],
                "thehat".into()
            ),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_stickers(
                vec!["notice".into(), "possible".into()],
                "basicbasic".into()
            ),
            -1
        );
    }
}
