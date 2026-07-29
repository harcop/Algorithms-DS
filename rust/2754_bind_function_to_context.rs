/// LeetCode #2754 - Bind Function to Context (JS problem; Rust analogue)
/// In Rust we model "binding a context" via a closure that captures the context.
fn bind<Ctx, F, Args, R>(f: F, ctx: Ctx) -> impl Fn(Args) -> R
where
    F: Fn(&Ctx, Args) -> R,
{
    move |args| f(&ctx, args)
}

fn main() {
    let bound = bind(|ctx: &i32, multiplier: i32| ctx * multiplier, 10);
    println!("{}", bound(5));
}

#[cfg(test)]
mod tests {
    use super::bind;

    #[test]
    fn example_one() {
        let bound = bind(|ctx: &i32, multiplier: i32| ctx * multiplier, 10);
        assert_eq!(bound(5), 50);
    }

    #[test]
    fn example_two() {
        let bound = bind(
            |ctx: &String, _: ()| format!("My name is {}", ctx),
            "Kathy".to_string(),
        );
        assert_eq!(bound(()), "My name is Kathy");
    }
}
