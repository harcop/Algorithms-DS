/// LeetCode #1419 - Minimum Number Of Frogs Croaking
fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
    let mut cnt = [0i32; 5];
    let mut frogs = 0i32;
    let mut active = 0i32;
    for c in croak_of_frogs.chars() {
        let i = match c {
            'c' => 0,
            'r' => 1,
            'o' => 2,
            'a' => 3,
            'k' => 4,
            _ => return -1,
        };
        if i == 0 {
            if cnt[4] > 0 {
                cnt[4] -= 1;
            } else {
                frogs += 1;
            }
            active += 1;
            cnt[0] += 1;
        } else {
            if cnt[i - 1] == 0 {
                return -1;
            }
            cnt[i - 1] -= 1;
            cnt[i] += 1;
            if c == 'k' {
                active -= 1;
            }
        }
    }
    if active == 0 { frogs } else { -1 }
}

fn main() {
    println!("{}", min_number_of_frogs("croakcroak".into()));
}

#[cfg(test)]
mod tests {
    use super::min_number_of_frogs;

    #[test]
    fn example_one() {
        assert_eq!(min_number_of_frogs("croakcroak".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_number_of_frogs("crcoakroak".into()), 2);
    }
}

