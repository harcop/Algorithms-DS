/// LeetCode #799 - Champagne Tower
fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    let mut row = vec![poured as f64];
    for _ in 0..query_row {
        let mut next = vec![0.0; row.len() + 1];
        for (i, &amount) in row.iter().enumerate() {
            let spill = (amount - 1.0).max(0.0) / 2.0;
            next[i] += spill;
            next[i + 1] += spill;
        }
        row = next;
    }
    row[query_glass as usize].min(1.0)
}

fn main() {
    println!("{}", champagne_tower(25, 6, 3));
}

#[cfg(test)]
mod tests {
    use super::champagne_tower;

    #[test]
    fn example_one() {
        let v = champagne_tower(25, 6, 3);
        assert!(v > 0.0 && v <= 1.0);
    }
}
