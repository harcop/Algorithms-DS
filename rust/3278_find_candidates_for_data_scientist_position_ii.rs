/// LeetCode #3278 - Find Candidates for Data Scientist Position II (SQL; Rust analogue)
use std::collections::{BTreeMap, HashMap};

fn find_candidates(
    candidates: Vec<(i32, String, i32)>,
    projects: Vec<(i32, String, i32)>,
) -> Vec<(i32, i32, i32)> {
    let mut cand: HashMap<i32, HashMap<String, i32>> = HashMap::new();
    for (id, skill, prof) in candidates {
        cand.entry(id).or_default().insert(skill, prof);
    }
    let mut proj: BTreeMap<i32, Vec<(String, i32)>> = BTreeMap::new();
    for (pid, skill, imp) in projects {
        proj.entry(pid).or_default().push((skill, imp));
    }
    let mut ans = Vec::new();
    for (pid, req) in proj {
        let mut best: Option<(i32, i32)> = None; // (score, -candidate_id) wait: highest score, then lowest id
        for (&cid, skills) in &cand {
            if !req.iter().all(|(s, _)| skills.contains_key(s)) {
                continue;
            }
            let mut score = 100;
            for (s, imp) in &req {
                let prof = skills[s];
                if prof > *imp {
                    score += 10;
                } else if prof < *imp {
                    score -= 5;
                }
            }
            match best {
                None => best = Some((score, cid)),
                Some((bs, bc)) if score > bs || (score == bs && cid < bc) => {
                    best = Some((score, cid));
                }
                _ => {}
            }
        }
        if let Some((score, cid)) = best {
            ans.push((pid, cid, score));
        }
    }
    ans
}

fn main() {
    let candidates = vec![
        (101, "Python".into(), 5),
        (101, "Tableau".into(), 3),
        (101, "PostgreSQL".into(), 4),
    ];
    let projects = vec![
        (501, "Python".into(), 4),
        (501, "Tableau".into(), 3),
        (501, "PostgreSQL".into(), 5),
    ];
    println!("{:?}", find_candidates(candidates, projects));
}

#[cfg(test)]
mod tests {
    use super::find_candidates;

    #[test]
    fn example() {
        let candidates = vec![
            (101, "Python".into(), 5),
            (101, "Tableau".into(), 3),
            (101, "PostgreSQL".into(), 4),
            (101, "TensorFlow".into(), 2),
            (102, "Python".into(), 4),
            (102, "Tableau".into(), 5),
            (102, "PostgreSQL".into(), 4),
            (102, "R".into(), 4),
            (103, "Python".into(), 3),
            (103, "Tableau".into(), 5),
            (103, "PostgreSQL".into(), 5),
            (103, "Spark".into(), 4),
        ];
        let projects = vec![
            (501, "Python".into(), 4),
            (501, "Tableau".into(), 3),
            (501, "PostgreSQL".into(), 5),
            (502, "Python".into(), 3),
            (502, "Tableau".into(), 4),
            (502, "R".into(), 2),
        ];
        assert_eq!(
            find_candidates(candidates, projects),
            vec![(501, 101, 105), (502, 102, 130)]
        );
    }
}
