/// LeetCode #1138 - Alphabet Board Path
fn alphabet_board_path(target: String) -> String {
    let mut r = 0i32;
    let mut c = 0i32;
    let mut out = String::new();
    for ch in target.chars() {
        let idx = (ch as u8 - b'a') as i32;
        let nr = idx / 5;
        let nc = idx % 5;
        if nr >= r {
            while c < nc {
                out.push('R');
                c += 1;
            }
            while c > nc {
                out.push('L');
                c -= 1;
            }
            while r < nr {
                out.push('D');
                r += 1;
            }
        } else {
            while r > nr {
                out.push('U');
                r -= 1;
            }
            while c < nc {
                out.push('R');
                c += 1;
            }
            while c > nc {
                out.push('L');
                c -= 1;
            }
        }
        out.push('!');
    }
    out
}

fn typed_from_path(path: &str) -> String {
    let mut r = 0i32;
    let mut c = 0i32;
    let mut out = String::new();
    for ch in path.chars() {
        match ch {
            'U' => r -= 1,
            'D' => r += 1,
            'L' => c -= 1,
            'R' => c += 1,
            '!' => out.push((b'a' + (r * 5 + c) as u8) as char),
            _ => {}
        }
    }
    out
}

fn main() {
    println!("{}", alphabet_board_path("leet".to_string()));
}

#[cfg(test)]
mod tests {
    use super::{alphabet_board_path, typed_from_path};

    #[test]
    fn example_one() {
        let path = alphabet_board_path("leet".to_string());
        assert_eq!(typed_from_path(&path), "leet");
    }

    #[test]
    fn example_two() {
        let path = alphabet_board_path("code".to_string());
        assert_eq!(typed_from_path(&path), "code");
    }
}
