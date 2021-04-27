// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

// See https://danielkeep.github.io/tlborm/book/mbe-min-import-export.html for
// #[macro_export]. Also see
// https://danielkeep.github.io/tlborm/book/mbe-min-scoping.html for a
// discussion of scoping rules (including the ordering of macro definitions vs.
// their usage, which matters, unlike regular functions).

fn main() {
    my_macro!();
}

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
