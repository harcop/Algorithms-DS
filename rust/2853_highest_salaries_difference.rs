/// LeetCode #2853 - Highest Salaries Difference
fn salary_difference(salaries: Vec<(String, String, i32)>) -> i32 {
    let mut engineering_max = i32::MIN;
    let mut marketing_max = i32::MIN;

    for (_, department, salary) in salaries {
        match department.as_str() {
            "Engineering" => engineering_max = engineering_max.max(salary),
            "Marketing" => marketing_max = marketing_max.max(salary),
            _ => {}
        }
    }

    engineering_max - marketing_max
}

fn main() {
    let salaries = vec![
        ("Kathy".into(), "Engineering".into(), 50_000),
        ("Roy".into(), "Marketing".into(), 30_000),
        ("Edward".into(), "Engineering".into(), 102_000),
        ("Evelyn".into(), "Marketing".into(), 53_000),
    ];
    println!("{}", salary_difference(salaries));
}

#[cfg(test)]
mod tests {
    use super::salary_difference;

    #[test]
    fn example_one() {
        let salaries = vec![
            ("Kathy".into(), "Engineering".into(), 50_000),
            ("Roy".into(), "Marketing".into(), 30_000),
            ("Charles".into(), "Engineering".into(), 45_000),
            ("Jack".into(), "Engineering".into(), 85_000),
            ("Benjamin".into(), "Marketing".into(), 34_000),
            ("Anthony".into(), "Marketing".into(), 42_000),
            ("Edward".into(), "Engineering".into(), 102_000),
            ("Terry".into(), "Engineering".into(), 44_000),
            ("Evelyn".into(), "Marketing".into(), 53_000),
            ("Arthur".into(), "Engineering".into(), 32_000),
        ];
        assert_eq!(salary_difference(salaries), 49_000);
    }
}
