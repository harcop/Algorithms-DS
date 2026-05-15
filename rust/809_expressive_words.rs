/// LeetCode #809 - Expressive Words
fn expressive_words(s: String, words: Vec<String>) -> i32 {
    fn groups(b: &[u8]) -> Vec<(u8, usize)> {
        let mut g = vec![];
        let mut i = 0usize;
        while i < b.len() {
            let c = b[i];
            let mut j = i;
            while j < b.len() && b[j] == c {
                j += 1;
            }
            g.push((c, j - i));
            i = j;
        }
        g
    }
    fn stretchy(s: &[u8], w: &[u8]) -> bool {
        let gs = groups(s);
        let gw = groups(w);
        if gs.len() != gw.len() {
            return false;
        }
        for (&(cs, ls), &(cw, lw)) in gs.iter().zip(gw.iter()) {
            if cs != cw {
                return false;
            }
            if ls < lw {
                return false;
            }
            if ls != lw && ls < 3 {
                return false;
            }
        }
        true
    }
    let sb = s.as_bytes();
    words.iter().filter(|w| stretchy(sb, w.as_bytes())).count() as i32
}

fn main() {
    println!(
        "{}",
        expressive_words("heeellooo".into(), vec!["hello".into(), "hi".into(), "helo".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::expressive_words;

    #[test]
    fn example_one() {
        assert_eq!(
            expressive_words(
                "heeellooo".into(),
                vec!["hello".into(), "hi".into(), "helo".into()]
            ),
            1
        );
    }
}
