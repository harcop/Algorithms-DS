/// LeetCode #2991 - Top Three Wineries (SQL; Rust analogue)
use std::collections::HashMap;

fn top_three_wineries(wineries: Vec<(i32, String, i32, String)>) -> Vec<(String, String, String, String)> {
    // (id, country, points, winery)
    let mut sums: HashMap<(String, String), i32> = HashMap::new();
    for (_, country, points, winery) in wineries {
        *sums.entry((country, winery)).or_default() += points;
    }
    let mut by_country: HashMap<String, Vec<(i32, String)>> = HashMap::new();
    for ((country, winery), points) in sums {
        by_country
            .entry(country)
            .or_default()
            .push((points, winery));
    }
    let mut ans = Vec::new();
    for (country, mut list) in by_country {
        list.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        let fmt = |i: usize| -> String {
            if i < list.len() {
                format!("{} ({})", list[i].1, list[i].0)
            } else if i == 1 {
                "No second winery".into()
            } else {
                "No third winery".into()
            }
        };
        ans.push((country, fmt(0), fmt(1), fmt(2)));
    }
    ans.sort_by(|a, b| a.0.cmp(&b.0));
    ans
}

fn main() {
    let wineries = vec![
        (103, "Australia".into(), 84, "WhisperingPines".into()),
        (737, "Australia".into(), 85, "GrapesGalore".into()),
        (848, "Australia".into(), 100, "HarmonyHill".into()),
        (222, "Hungary".into(), 60, "MoonlitCellars".into()),
        (116, "USA".into(), 47, "RoyalVines".into()),
        (124, "USA".into(), 45, "Eagle'sNest".into()),
        (648, "India".into(), 69, "SunsetVines".into()),
        (894, "USA".into(), 39, "RoyalVines".into()),
        (677, "USA".into(), 9, "PacificCrest".into()),
    ];
    println!("{:?}", top_three_wineries(wineries));
}

#[cfg(test)]
mod tests {
    use super::top_three_wineries;

    #[test]
    fn example() {
        let wineries = vec![
            (103, "Australia".into(), 84, "WhisperingPines".into()),
            (737, "Australia".into(), 85, "GrapesGalore".into()),
            (848, "Australia".into(), 100, "HarmonyHill".into()),
            (222, "Hungary".into(), 60, "MoonlitCellars".into()),
            (116, "USA".into(), 47, "RoyalVines".into()),
            (124, "USA".into(), 45, "Eagle'sNest".into()),
            (648, "India".into(), 69, "SunsetVines".into()),
            (894, "USA".into(), 39, "RoyalVines".into()),
            (677, "USA".into(), 9, "PacificCrest".into()),
        ];
        assert_eq!(
            top_three_wineries(wineries),
            vec![
                (
                    "Australia".into(),
                    "HarmonyHill (100)".into(),
                    "GrapesGalore (85)".into(),
                    "WhisperingPines (84)".into()
                ),
                (
                    "Hungary".into(),
                    "MoonlitCellars (60)".into(),
                    "No second winery".into(),
                    "No third winery".into()
                ),
                (
                    "India".into(),
                    "SunsetVines (69)".into(),
                    "No second winery".into(),
                    "No third winery".into()
                ),
                (
                    "USA".into(),
                    "RoyalVines (86)".into(),
                    "Eagle'sNest (45)".into(),
                    "PacificCrest (9)".into()
                ),
            ]
        );
    }
}
