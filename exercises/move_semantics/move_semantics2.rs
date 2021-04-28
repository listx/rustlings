// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

// I AM NOT DONE

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0);

    // The key difference with this exercise vs. move_semantics1 is that we're
    // trying to use vec0 here (dereference it to the underlying bytes that it
    // points to) in main(). The compiler is telling us though that in line 10
    // above, vec0's value (the heap-allocated bytes it points to) get *moved*
    // into fill_vec().
    //
    // The thing to remember is that whenever a move occurs, essentially the
    // original variable is destroyed (becomes invalid). This is explained in
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move
    // (please read this subsection in its entirety as it has pictures and
    // everything to make it easy!). So here, vec0 is the original pointer to
    // the vector's memory. Inside fill_vec(), we create a new pointer that
    // points to the same area, so at that point, vec0 becomes invalidated.
    // That's why the compiler complains that a move occurs at line 10.
    //
    // Q: We still really want to use vec0 here. But a move occurred, which
    // invalidates vec0. How can we make it so that a move (shallow copy) does
    // *NOT* occur?
    //
    // There are 3 ways suggested by "rustlings hint move_semantics2":
    // 1. Make another, separate version of the data that's in `vec0` and pass
    //    that to `fill_vec` instead.
    // 2. Make `fill_vec` borrow its argument instead of taking ownership of it,
    //    and then copy the data within the function in order to return an owned
    //    `Vec<i32>`
    // 3. Make `fill_vec` *mutably* borrow its argument (which will need to be
    //    mutable), modify it directly, then not return anything. Then you can
    //    get rid of `vec1` entirely -- note that this will change what gets
    //    printed by the first `println!`

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // OUTPUT:
    //
    //    vec0 has length 0 content `[]`
    //    vec1 has length 4 content `[22, 44, 66, 88]`
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    // let mut vec = vec;
    //    vec.push(22);
    //    vec.push(44);
    //    vec.push(66);

    //    vec

    // Approach 2 (from "rustlings hint move_semantics2"): Make fill_vec() use
    // the vec argument as a starting point for a completely new vector. So just
    // clone the given one, then add into this new cloned copy. This way we
    // avoid the move because we take a reference (& symbol) and Rust knows that
    // we've only temporarily borrowed the argument, so that the original
    // argument is *NOT* dropped when fill_vec() is called.
    //
    // The easiest way to use vec as a starting point is to clone it so that we
    // get whatever it has inside it.
    let mut vec_new = vec.clone();

    vec_new.push(22);
    vec_new.push(44);
    vec_new.push(66);

    vec_new
}
