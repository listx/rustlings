// arc1.rs
// In this exercise, we are given a Vec of u32 called "numbers" with values ranging
// from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ]
// We would like to use this set of numbers within 8 different threads simultaneously.
// Each thread is going to get the sum of every eighth value, with an offset.
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...

// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.


// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the `numbers` Vec!
// Execute `rustlings hint arc1` for hints :)

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    // 'numbers' holds a vector of 100 numbers, 1 through 100.
    let numbers: Vec<_> = (1..101u32).collect();
    // 'shared numbers' is a smart pointer to the 'numbers' data on the head,
    // which is atomically reference-counted (Arc).
    let shared_numbers = Arc::new(numbers);
    // 'joinhandles' is a vector of 'JoinHandle's, which are created by the
    // calls to thread::spawn().
    let mut joinhandles = Vec::new();

    // The exercise originally makes each thread compute a sum, by taking every
    // 8th number in the 'numbers' vector, but with the twist of assigning each
    // thread a starting offset from 0. This is fine, but the sums are hard to
    // grok. So instead, here we take the liberty of defining a more easily
    // verifiable behavior: make each thread collect every nth integer. E.g.,
    // thread '1' will find all numbers in the vector, thread '2' will find
    // every 2nd number, thread '3' will find every 3rd number, etc.
    for offset in 1..9 {
        // 'child_numbers' now points to the same memory location as
        // shared_numbers.
        let child_numbers = Arc::clone(&shared_numbers);
        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut vec = Vec::new();
            while i <= child_numbers.len() {
                vec.push(child_numbers[i - 1]);
                i += offset;
            }
            println!("Thread {}'s vector: {:?}", offset, vec);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
