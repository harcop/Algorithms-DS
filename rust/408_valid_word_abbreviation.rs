/// LeetCode #408 - Valid Word Abbreviation
fn valid_word_abbreviation(word: String, abbr: String) -> bool {
    let w = word.into_bytes();
    let a = abbr.into_bytes();
    let mut i = 0usize;
    let mut j = 0usize;
    while i < w.len() && j < a.len() {
        if a[j].is_ascii_digit() {
            if a[j]==b'0' { return false; }
            let mut num = 0usize;
            while j < a.len() && a[j].is_ascii_digit() {
                num=num*10 + (a[j]-b'0') as usize;
                j+=1;
            }
            i+=num;
        } else {
            if w[i]!=a[j] { return false; }
            i+=1; j+=1;
        }
    }
    i==w.len() && j==a.len()
}

fn main(){ println!("{}", valid_word_abbreviation("internationalization".into(), "i12iz4n".into())); }

#[cfg(test)] mod tests { use super::*;
    #[test] fn ex(){ assert!(valid_word_abbreviation("internationalization".into(), "i12iz4n".into())); }
}
