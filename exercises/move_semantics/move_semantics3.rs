// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)

// I AM NOT DONE

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// Here we are quite simply modifying the underlying memory pointed by vec,
// by calling push(). As it stands though, the declaration in the parameter
// list "vec: Vec<i32>" states that "vec" is an immutable variable. And so
// either we have to perform only non-mutating (read-only) operations on it,
// or, if we want to keep the push() calls to modify it, declare that we
// expect it to be mutable instead.
//
// Q: How do we make "vec" mutable?
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
