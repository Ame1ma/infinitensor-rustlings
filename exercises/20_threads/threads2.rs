// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{sync::Arc, thread, time::Duration};

struct JobStatus {
    jobs_done: std::sync::atomic::AtomicU32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(JobStatus {
        jobs_done: std::sync::atomic::AtomicU32::new(0),
    });

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value.
            status_shared
                .jobs_done
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    println!(
        "Jobs done: {}",
        status.jobs_done.load(std::sync::atomic::Ordering::Relaxed)
    );
}
