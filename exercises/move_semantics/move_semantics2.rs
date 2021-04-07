// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let mut vec0 = Vec::new();

    vec0 = fill_vec(vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // If we move this assignment *above* the println!() macro above,
    // compilation fails because the assignment causes the move to occur from
    // vec0 to vec1, rendering vec0 unusable (because it is DROPPED at the point
    // of the move).
    let mut vec1 = vec0;

    // This will fail because the value of vec0 has moved to the vec1 reference.
    // `vec0` is now basically the same thing as a NULL pointer in C/C++, such
    // that trying to dereference it is an error.
    // println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // --- fill_vec_borrow() ---
    println!("\nfill_vec_borrow:");
    let vec0 = Vec::new();

    let mut vec1 = fill_vec_borrow(&vec0);

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // --- fill_vec_mut_borrow() ---
    println!("\nfill_vec_mut_borrow:");
    let mut vec0 = Vec::new();

    fill_vec_mut_borrow(&mut vec0);

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

fn fill_vec(inputVec: Vec<i32>) -> Vec<i32> {
    // Here we choose to *MODIFY* the vector by creating a mutable reference to
    // it, and then acting with this mutable reference to call .push(). Because
    // we're modifying the data, we are taking ownership of it directly. So
    // whenever anyone calls us, we take ownership outright.
    //
    // In this case, main() first owns vec0, because it's created there. However
    // when it calls fill_vec(), fill_vec() takes ownership of it and at that
    // point main() is prohibited from modifying vec0 again using the same vec0
    // mutable reference.
    //
    // However, main() can create another new reference called vec1 that is
    // based on vec0, and modify that instead. vec0 is DROPPED when `let mut
    // vec1 = vec0;` is executed, and can no longer be referenced. To prove
    // this, you can try printing vec0's contents after this happens (this will
    // fail).
    let mut vec = inputVec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

// Just borrow the inputVec, and don't modify it. Borrowing inputVec (instead of
// owning it) tells Rust to *AVOID MOVING* the data (taking ownership of the
// data) to the `inputVec` reference when we call it.
//
// IOW, we get a readonly view of the inputVec, and then create a new vector
// that we own.
fn fill_vec_borrow(inputVec: &Vec<i32>) -> Vec<i32> {
    let mut newVec = Vec::new();

    // Perform a manual copy of inputVec's contents into newVec.
    for x in inputVec.iter() {
        // We have to dereference the 'x' binding here because iter() gives us a
        // borrowed reference to each element in the vector.
        newVec.push(*x);
    }

    newVec.push(3);
    newVec.push(33);
    newVec.push(333);

    newVec
}

// *MUTABLY* borrow inputVec. This gives us the power to directly modify
// inputVec. Because we're directly modifying the input, there's no need to
// return anything.
fn fill_vec_mut_borrow(inputVec: &mut Vec<i32>) -> () {
    inputVec.push(9);
    inputVec.push(99);
    inputVec.push(999);
}
