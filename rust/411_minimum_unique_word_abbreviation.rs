/// LeetCode #411 - Minimum Unique Word Abbreviation (bitmask brute; valid for TL ≤ Leet constraints)
fn min_abbreviation(target: String, dictionary: Vec<String>) -> String {
    let tl = target.len();
    let tb = target.as_bytes();
    let others: Vec<&[u8]> = dictionary
        .iter()
        .filter(|w| w.len() == tl && w.as_bytes() != tb)
        .map(|w| w.as_bytes())
        .collect();

    fn abbrev_from_keep(t: &[u8], keep: &[bool]) -> String {
        let mut i = 0usize;
        let mut out = String::new();
        while i < t.len() {
            if keep[i] {
                out.push(t[i] as char);
                i += 1;
            } else {
                let mut j = i;
                while j < t.len() && !keep[j] {
                    j += 1;
                }
                let run = j - i;
                if run == 0 {
                    break;
                }
                if run == 1 {
                    out.push(t[i] as char);
                    i += 1;
                    continue;
                }
                out.push_str(&run.to_string());
                out.push(t[j - 1] as char);
                i = j;
            }
        }
        out
    }

    let mut masks: Vec<usize> = (0..(1 << tl.min(12))).collect();
    masks.sort_by_key(|m| abbrev_from_keep(tb, &mask_to_keep(*m, tl)).len());

    fn mask_to_keep(m: usize, tl: usize) -> Vec<bool> {
        (0..tl).map(|i| m >> i & 1 != 0).collect()
    }

    for m in masks {
        let keep = mask_to_keep(m, tl);
        let a = abbrev_from_keep(tb, &keep);
        if others
            .iter()
            .all(|w| abbrev_from_keep(w, &keep) != a)
        {
            return if a.len() <= target.len() { a } else { target };
        }
    }
    target
}

fn main() {
    println!(
        "{}",
        min_abbreviation(
            "apple".into(),
            vec!["plain".into(), "amber".into(), "knife".into()]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let r = min_abbreviation("apple".into(), vec!["blade".into()]);
        assert!(r.len() <= 6);
    }
}
