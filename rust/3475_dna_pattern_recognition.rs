/// LeetCode #3475 - DNA Pattern Recognition (SQL; Rust analogue)
fn analyze_dna_patterns(
    mut samples: Vec<(i32, String, String)>,
) -> Vec<(i32, String, String, i32, i32, i32, i32)> {
    samples.sort_by_key(|(id, _, _)| *id);
    samples
        .into_iter()
        .map(|(id, dna, species)| {
            let has_start = dna.starts_with("ATG") as i32;
            let has_stop = (dna.ends_with("TAA") || dna.ends_with("TAG") || dna.ends_with("TGA"))
                as i32;
            let has_atat = dna.contains("ATAT") as i32;
            let has_ggg = dna.contains("GGG") as i32;
            (id, dna, species, has_start, has_stop, has_atat, has_ggg)
        })
        .collect()
}

fn main() {
    let samples = vec![
        (1, "ATGCTAGCTAGCTAA".into(), "Human".into()),
        (2, "GGGTCAATCATC".into(), "Human".into()),
    ];
    println!("{:?}", analyze_dna_patterns(samples));
}

#[cfg(test)]
mod tests {
    use super::analyze_dna_patterns;

    #[test]
    fn example() {
        let samples = vec![
            (1, "ATGCTAGCTAGCTAA".into(), "Human".into()),
            (2, "GGGTCAATCATC".into(), "Human".into()),
            (3, "ATATATCGTAGCTA".into(), "Human".into()),
            (4, "ATGGGGTCATCATAA".into(), "Mouse".into()),
            (5, "TCAGTCAGTCAG".into(), "Mouse".into()),
            (6, "ATATCGCGCTAG".into(), "Zebrafish".into()),
            (7, "CGTATGCGTCGTA".into(), "Zebrafish".into()),
        ];
        assert_eq!(
            analyze_dna_patterns(samples),
            vec![
                (1, "ATGCTAGCTAGCTAA".into(), "Human".into(), 1, 1, 0, 0),
                (2, "GGGTCAATCATC".into(), "Human".into(), 0, 0, 0, 1),
                (3, "ATATATCGTAGCTA".into(), "Human".into(), 0, 0, 1, 0),
                (4, "ATGGGGTCATCATAA".into(), "Mouse".into(), 1, 1, 0, 1),
                (5, "TCAGTCAGTCAG".into(), "Mouse".into(), 0, 0, 0, 0),
                (6, "ATATCGCGCTAG".into(), "Zebrafish".into(), 0, 1, 1, 0),
                (7, "CGTATGCGTCGTA".into(), "Zebrafish".into(), 0, 0, 0, 0),
            ]
        );
    }
}
