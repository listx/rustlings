// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

fn main() {
    // This can work:
    // let mut shopping_list: Vec<&'static str> = Vec::new();
    // Or let the compiler infer it:
    let mut shopping_list: Vec<_> = Vec::new();
    shopping_list.push("milk");
}
