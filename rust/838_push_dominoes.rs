/// LeetCode #838 - Push Dominoes
fn push_dominoes(dominoes: String) -> String {
    let s: Vec<char> = dominoes.chars().collect();
    let n = s.len();
    let mut force = vec![0i32; n];
    let mut f = 0i32;
    for i in 0..n {
        if s[i] == 'R' {
            f = n as i32;
        } else if s[i] == 'L' {
            f = 0;
        } else {
            f = f.saturating_sub(1);
        }
        force[i] += f;
    }
    f = 0;
    for i in (0..n).rev() {
        if s[i] == 'L' {
            f = n as i32;
        } else if s[i] == 'R' {
            f = 0;
        } else {
            f = f.saturating_sub(1);
        }
        force[i] -= f;
    }
    force
        .iter()
        .map(|&x| {
            if x > 0 {
                'R'
            } else if x < 0 {
                'L'
            } else {
                '.'
            }
        })
        .collect()
}

fn main() {
    println!("{}", push_dominoes("RR.L".into()));
}

#[cfg(test)]
mod tests {
    use super::push_dominoes;

    #[test]
    fn example_one() {
        assert_eq!(push_dominoes("RR.L".into()), "RR.L");
    }

    #[test]
    fn example_two() {
        assert_eq!(push_dominoes("L.RR.LL.RRLLL.".into()), "L.RR.LL.RRLLL.");
    }
}
