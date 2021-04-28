// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

// I AM NOT DONE

fn main() {
    // Approach 3 (from "rustlings hint move_semantics2"): Just create a single
    // mutable variable, and make fill_vec() grow it directly. So now
    // fill_vec() expects a mutable reference to borrow (no new vectors are
    // being created in this program). In other words, the other 2 approaches
    // involve a deep copy, but this one does not. It doesn't even involve
    // shallow copies, because we only have 1 pointer (vec0) that we pass
    // around. As per the hint, this makes vec1 needless as there's no point in
    // having 2 pointers around for the *same* bytes in memory when we can just
    // have 1. Hence the removal of vec1.
    let mut vec0 = Vec::new();

    fill_vec(&mut vec0);

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

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // OUTPUT:
    //
    //    vec0 has length 3 content `[22, 44, 66]`
    //    vec0 has length 4 content `[22, 44, 66, 88]`
}

fn fill_vec(vec: &mut Vec<i32>) -> () {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
