/// LeetCode #3175 - Find The First Player to win K Games in a Row
fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
    let n = skills.len();
    let k = k.min((n - 1) as i32);
    let mut i = 0usize;
    let mut cnt = 0;
    for j in 1..n {
        if skills[i] < skills[j] {
            i = j;
            cnt = 1;
        } else {
            cnt += 1;
        }
        if cnt == k {
            break;
        }
    }
    i as i32
}

fn main() {
    println!("{}", find_winning_player(vec![4, 2, 6, 3, 9], 2));
}

#[cfg(test)]
mod tests {
    use super::find_winning_player;

    #[test]
    fn example1() {
        assert_eq!(find_winning_player(vec![4, 2, 6, 3, 9], 2), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(find_winning_player(vec![2, 5, 4], 3), 1);
    }
}
