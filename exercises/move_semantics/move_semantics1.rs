// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

fn main() {
    let vec0 = Vec::new();

    // We have to declare vec1 to be a mutable reference, because we (the main()
    // function) are going to modify it below with `vec1.push(88)`. The .push()
    // method (https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)
    // stipulates that it only takes a mutable reference. So trying to use an
    // immutable reference with it is a natural compiler error.
    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
