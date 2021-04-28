// move_semantics1.rs
// Make me compile! Execute `rustlings hint move_semantics1` for hints :)

// I AM NOT DONE

fn main() {
    // Here vec0 is an immutable variable containing a vector (basically a
    // dynamic array). From here on out, vec0 cannot be used to modify the
    // underlying memory in the vector (even though the memory is heap-allocated
    // and totally supports growing and shrinking based on runtime usage).
    // Anyway, making vec0 immutable here is OK because we don't use vec0 to
    // modify the vector. So, someone else is free to grow the vector, just not
    // us.
    let vec0 = Vec::new();

    // Indeed, we just pass along the value to fill_vec(), which can grow it.
    // Even though fill_vec() will grow the vector, it's OK because fill_vec()
    // is the one doing the growing, NOT main().
    //
    // When fill_vec() is done it returns the modified vector back to us
    // (pointing to the same region of heap memory pointed-to by vec0).
    //
    // When we create vec1, we're saying that we still want to make it
    // immutable. There's nothing wrong with this per-se, it's just that we
    // break this contract by trying to modify it below with vec1.push(88).
    //
    // Q: how can we declare a different intent here so that we can mutate
    // the underlying vector within main() directly?
    let vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// Take a vector, then grow it with 3 more elements.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
