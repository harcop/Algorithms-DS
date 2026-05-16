/// LeetCode #843 - Guess the Word
fn find_secret_word(wordlist: Vec<String>, master: &dyn Master) {
    let mut candidates = wordlist;
    while !candidates.is_empty() {
        let guess = candidates[0].clone();
        let matches = master.guess(&guess);
        if matches == 6 {
            return;
        }
        candidates.retain(|w| {
            let same = guess
                .bytes()
                .zip(w.bytes())
                .filter(|(a, b)| a == b)
                .count();
            same == matches as usize
        });
    }
}

trait Master {
    fn guess(&self, word: &str) -> i32;
}

struct MockMaster {
    secret: String,
}

impl Master for MockMaster {
    fn guess(&self, word: &str) -> i32 {
        word.bytes()
            .zip(self.secret.bytes())
            .filter(|(a, b)| a == b)
            .count() as i32
    }
}

fn main() {
    let words = vec![
        "acckzz".into(),
        "ccbazz".into(),
        "eiowzz".into(),
        "abhbzz".into(),
    ];
    let master = MockMaster {
        secret: "acckzz".into(),
    };
    find_secret_word(words, &master);
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::{find_secret_word, Master, MockMaster};

    #[test]
    fn example_one() {
        let words = vec![
            "acckzz".into(),
            "ccbazz".into(),
            "eiowzz".into(),
            "abhbzz".into(),
        ];
        let master = MockMaster {
            secret: "acckzz".into(),
        };
        find_secret_word(words, &master);
    }
}
