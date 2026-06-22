/// LeetCode #2038 - Remove Colored Pieces if Both Neighbors are the Same Color
fn winner_of_game(colors: String) -> bool {
    let colors = colors.as_bytes();
    let mut a = 0i32;
    let mut b = 0i32;
    let mut i = 0usize;
    while i < colors.len() {
        let c = colors[i];
        let mut j = i;
        while j < colors.len() && colors[j] == c {
            j += 1;
        }
        let m = (j - i) as i32 - 2;
        if m > 0 {
            if c == b'A' {
                a += m;
            } else {
                b += m;
            }
        }
        i = j;
    }
    a > b
}

fn main() {
    println!("{}", winner_of_game("AAABABB".into()));
}

#[cfg(test)]
mod tests {
    use super::winner_of_game;

    #[test]
    fn example_one() {
        assert!(winner_of_game("AAABABB".into()));
    }

    #[test]
    fn example_two() {
        assert!(!winner_of_game("AA".into()));
    }

    #[test]
    fn example_three() {
        assert!(!winner_of_game("ABBBBBBBAA".into()));
    }
}
