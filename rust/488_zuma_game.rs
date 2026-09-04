/// LeetCode #488 - Zuma Game
use std::collections::HashMap;

fn find_min_step(board: String, hand: String) -> i32 {
    let mut hand_cnt = [0i32; 26];
    for b in hand.bytes() {
        hand_cnt[(b - b'A') as usize] += 1;
    }
    let mut memo = HashMap::new();
    let board = clean(&board);
    let ans = dfs(&board, &mut hand_cnt, &mut memo);
    if ans >= 6 {
        -1
    } else {
        ans
    }
}

fn clean(s: &str) -> String {
    let mut cur = s.to_string();
    loop {
        let b: Vec<char> = cur.chars().collect();
        if b.is_empty() {
            return cur;
        }
        let mut out = String::new();
        let mut i = 0;
        let mut changed = false;
        while i < b.len() {
            let mut j = i;
            while j < b.len() && b[j] == b[i] {
                j += 1;
            }
            if j - i >= 3 {
                changed = true;
            } else {
                for k in i..j {
                    out.push(b[k]);
                }
            }
            i = j;
        }
        cur = out;
        if !changed {
            return cur;
        }
    }
}

fn dfs(board: &str, hand: &mut [i32; 26], memo: &mut HashMap<(String, [i32; 26]), i32>) -> i32 {
    if board.is_empty() {
        return 0;
    }
    let key = (board.to_string(), *hand);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let mut ans = 6;
    let b: Vec<char> = board.chars().collect();
    let mut i = 0;
    while i < b.len() {
        let mut j = i;
        while j < b.len() && b[j] == b[i] {
            j += 1;
        }
        let color = (b[i] as u8 - b'A') as usize;
        let need = (3 - (j - i) as i32).max(1);
        if hand[color] >= need {
            hand[color] -= need;
            let mut nb = String::new();
            nb.extend(b[..i].iter());
            nb.extend(b[j..].iter());
            let nb = clean(&nb);
            ans = ans.min(need + dfs(&nb, hand, memo));
            hand[color] += need;
        }
        i = j;
    }
    memo.insert(key, ans);
    ans
}

fn main() {
    println!("{}", find_min_step("WRRBBW".into(), "RB".into()));
}

#[cfg(test)]
mod tests {
    use super::find_min_step;

    #[test]
    fn example_one() {
        assert_eq!(find_min_step("WRRBBW".into(), "RB".into()), -1);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_min_step("WWRRBBWW".into(), "WRBRW".into()), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_min_step("G".into(), "GGGGG".into()), 2);
    }
}
