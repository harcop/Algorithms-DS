/// LeetCode #470 - Implement Rand10() Using Rand7()

fn rand10<F: FnMut() -> i32>(mut rand7: F) -> i32 {
    loop {
        let a = rand7() - 1;
        let b = rand7() - 1;
        let x = a * 7 + b;
        if x < 40 {
            return x % 10 + 1;
        }
    }
}

fn main() {
    let mut calls = 0;
    let mut stream = [4, 7, 5, 3, 6, 2, 1].iter().cycle();
    let r = rand10(|| {
        calls += 1;
        *stream.next().unwrap()
    });
    println!("{} ({})", r, calls);
}

#[cfg(test)]
mod tests {
    use super::rand10;

    #[test]
    fn in_range() {
        let mut i = 0u32;
        let vals = [3, 7, 4, 6, 5, 2, 1, 7, 3, 4, 6, 5, 7, 2, 1];
        let r = rand10(|| {
            let v = vals[i as usize % vals.len()];
            i += 1;
            v
        });
        assert!((1..=10).contains(&r));
    }
}
