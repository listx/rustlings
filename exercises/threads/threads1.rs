// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

// Mutex gives us the lock() method which lets us acquire a region of memory
// while denying it from other threads.
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // We use a Mutex to protect shared data. The "status" and "shared_status"
    // variables both point to the same region of memory, so we use a Mutex to
    // protect it. This also allows us to mutate that region of memory (mutation
    // is implied, and is why we don't have to declare a mutable reference from
    // inside the spawned thread below); see
    // https://doc.rust-lang.org/std/sync/struct.Arc.html, which states:
    //
    //     If you need to mutate through an Arc, use Mutex, RwLock, or one of
    //     the Atomic types.
    //
    // This thought is echoed again in
    // https://doc.rust-lang.org/book/ch16-03-shared-state.html#similarities-between-refcelltrct-and-mutextarct,
    // which says:
    //
    //     ...this means Mutex<T> provides interior mutability, as the Cell
    //     family does. In the same way we used RefCell<T> in Chapter 15 to allow us
    //     to mutate contents inside an Rc<T>, we use Mutex<T> to mutate contents
    //     inside an Arc<T>.
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    // "status_shared" refers to the *SAME* region of memory as "status". See
    // https://doc.rust-lang.org/std/sync/struct.Arc.html#cloning-references.
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(25));
            // Acquire the region of memory from other threads (in this case,
            // the main thread). If we can't acquire it, we panic().
            status_shared.lock().unwrap().jobs_completed += 1;
        }
    });

    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(50));
    }

    // Create a mutable reference to JobStatus (just for fun). We can mutate
    // freely now because the spawned thread above has terminated.
    let mut x = status.lock().unwrap();
    println!("{}", x.jobs_completed);
    x.jobs_completed += 1;
    println!("{}", x.jobs_completed);
    x.jobs_completed += 1;
    println!("{}", x.jobs_completed);
}
