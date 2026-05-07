/// LeetCode #332 - Reconstruct Itinerary (Hierholzer; lex smallest)
use std::collections::HashMap;

fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for t in tickets {
        graph.entry(t[0].clone()).or_default().push(t[1].clone());
    }
    for neigh in graph.values_mut() {
        neigh.sort_unstable_by(|a, b| b.cmp(a));
    }
    let mut route: Vec<String> = Vec::new();
    fn dfs(g: &mut HashMap<String, Vec<String>>, u: &str, route: &mut Vec<String>) {
        while let Some(v) = g.get_mut(u).and_then(|nei| nei.pop()) {
            dfs(g, &v, route);
        }
        route.push(u.to_string());
    }
    dfs(&mut graph, "JFK", &mut route);
    route.reverse();
    route
}

fn main() {
    println!(
        "{:?}",
        find_itinerary(vec![
            vec!["MUC".into(), "LHR".into()],
            vec!["JFK".into(), "MUC".into()],
            vec!["SFO".into(), "SJC".into()],
            vec!["LHR".into(), "SFO".into()],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::find_itinerary;

    #[test]
    fn examples() {
        assert_eq!(
            find_itinerary(vec![
                vec!["MUC".into(), "LHR".into()],
                vec!["JFK".into(), "MUC".into()],
                vec!["SFO".into(), "SJC".into()],
                vec!["LHR".into(), "SFO".into()],
            ]),
            vec!["JFK", "MUC", "LHR", "SFO", "SJC"]
        );

        assert_eq!(
            find_itinerary(vec![
                vec!["JFK".into(), "SFO".into()],
                vec!["JFK".into(), "ATL".into()],
                vec!["SFO".into(), "ATL".into()],
                vec!["ATL".into(), "JFK".into()],
                vec!["ATL".into(), "SFO".into()],
            ]),
            vec!["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]
        );
    }
}
