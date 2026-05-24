/// LeetCode #1257 - Smallest Common Region
use std::collections::HashMap;

fn find_smallest_region(regions: Vec<Vec<String>>, region1: String, region2: String) -> String {
    let mut parent: HashMap<String, String> = HashMap::new();
    for group in &regions {
        for i in 1..group.len() {
            parent.insert(group[i].clone(), group[0].clone());
        }
    }
    fn ancestors(x: &str, parent: &HashMap<String, String>) -> Vec<String> {
        let mut path = vec![x.to_string()];
        let mut cur = x.to_string();
        while let Some(p) = parent.get(&cur) {
            path.push(p.clone());
            cur = p.clone();
        }
        path
    }
    let a1 = ancestors(&region1, &parent);
    let a2: std::collections::HashSet<_> = ancestors(&region2, &parent).into_iter().collect();
    for r in a1 {
        if a2.contains(&r) {
            return r;
        }
    }
    region1
}

fn main() {
    let regions = vec![
        vec![
            "Earth".into(),
            "North America".into(),
            "South America".into(),
        ],
        vec![
            "North America".into(),
            "United States".into(),
            "Canada".into(),
        ],
        vec![
            "United States".into(),
            "New York".into(),
            "Boston".into(),
        ],
        vec!["Canada".into(), "Ontario".into(), "Quebec".into()],
        vec!["South America".into(), "Brazil".into()],
    ];
    println!(
        "{}",
        find_smallest_region(regions, "Quebec".into(), "New York".into())
    );
}

#[cfg(test)]
mod tests {
    use super::find_smallest_region;

    fn sample_regions() -> Vec<Vec<String>> {
        vec![
            vec![
                "Earth".into(),
                "North America".into(),
                "South America".into(),
            ],
            vec![
                "North America".into(),
                "United States".into(),
                "Canada".into(),
            ],
            vec![
                "United States".into(),
                "New York".into(),
                "Boston".into(),
            ],
            vec!["Canada".into(), "Ontario".into(), "Quebec".into()],
            vec!["South America".into(), "Brazil".into()],
        ]
    }

    #[test]
    fn example_one() {
        assert_eq!(
            find_smallest_region(sample_regions(), "Quebec".into(), "New York".into()),
            "North America".to_string()
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_smallest_region(sample_regions(), "Canada".into(), "United States".into()),
            "North America".to_string()
        );
    }
}
