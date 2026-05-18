/// LeetCode #948 - Bag of Tokens

fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
    let mut tokens = tokens;
    tokens.sort_unstable();
    let mut lo = 0usize;
    let mut hi = tokens.len();
    let mut p = power;
    let mut score = 0i32;
    let mut best = 0i32;
    while lo < hi {
        if p >= tokens[lo] {
            p -= tokens[lo];
            lo += 1;
            score += 1;
            best = best.max(score);
        } else if score > 0 {
            p += tokens[hi - 1];
            hi -= 1;
            score -= 1;
        } else {
            break;
        }
    }
    best
}

fn main() {
    println!("{}", bag_of_tokens_score(vec![100, 200, 300, 400], 200));
}

#[cfg(test)]
mod tests {
    use super::bag_of_tokens_score;

    #[test]
    fn example_one() {
        assert_eq!(bag_of_tokens_score(vec![100, 200, 300, 400], 200), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(bag_of_tokens_score(vec![100, 200], 150), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(bag_of_tokens_score(vec![100, 200, 300], 150), 1);
    }
}
