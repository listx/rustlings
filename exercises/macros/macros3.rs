// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

// See https://danielkeep.github.io/tlborm/book/mbe-min-import-export.html for
// info about #[macro_use].
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
