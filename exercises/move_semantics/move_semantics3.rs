// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)

// I AM NOT DONE

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    // Uncommenting this line will break compilation!
    //    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// Here we are quite simply modifying the underlying memory pointed by vec,
// by calling push(). As it stands though, the declaration in the parameter
// list "vec: Vec<i32>" states that "vec" is an immutable reference. And so
// either we have to perform only non-mutating (read-only) operations on it,
// or, if we want to keep the push() calls to modify it, declare that we
// expect to take a mutable reference to it instead.
//
// Q: How do we make "vec" mutable?
// A: Just add the "mut" keyword to make the argument "mut vec: Vec<i32>". Note
// however, that up in main(), we cannot use vec0 again after the call to
// fill_vec(), because a *move* occurs in the call to fill_vec(). If we want to
// take a mutable borrow (not a move) instead, see the solution to the 3rd
// answer variation in the previous exercise).
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
