/// LeetCode #2029 - Stone Game IX
fn stone_game_ix(stones: Vec<i32>) -> bool {
    fn check(mut cnt: [i32; 3]) -> bool {
        if cnt[1] == 0 {
            return false;
        }
        cnt[1] -= 1;
        let mut r = 1 + cnt[1].min(cnt[2]) * 2 + cnt[0];
        if cnt[1] > cnt[2] {
            cnt[1] -= 1;
            r += 1;
        }
        r % 2 == 1 && cnt[1] != cnt[2]
    }

    let mut c1 = [0i32; 3];
    for x in stones {
        c1[(x % 3) as usize] += 1;
    }
    let c2 = [c1[0], c1[2], c1[1]];
    check(c1) || check(c2)
}

fn main() {
    println!("{}", stone_game_ix(vec![2, 1]));
}

#[cfg(test)]
mod tests {
    use super::stone_game_ix;

    #[test]
    fn example_one() {
        assert!(stone_game_ix(vec![2, 1]));
    }

    #[test]
    fn example_two() {
        assert!(!stone_game_ix(vec![2]));
    }

    #[test]
    fn example_three() {
        assert!(!stone_game_ix(vec![5, 1, 2, 4, 3]));
    }
}
