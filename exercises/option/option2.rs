// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

fn main() {
    let optional_word = Some(String::from("rustlings"));
    // See https://doc.rust-lang.org/book/ch06-03-if-let.html for "if let"
    // syntax, which is a sort of simplified "match" expression.
    if let Some(word) = optional_word {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // The outer Some(..) is from the use of vec.pop(). The inner Some(..) is
    // from the "Some(x)" from above.
    while let Some(Some(integer)) = optional_integers_vec.pop() {
        println!("current value: {}", integer);
    }
}
