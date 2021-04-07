// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    // `num * num` is an expression.
    num * num
    // Alternative: `return num * num;`, which is a statement. Statements
    // (denoted by a semicolon) do not evaluate to a value by themselves, so if
    // we want the value out of it, we have to use the `return` keyword.
}
