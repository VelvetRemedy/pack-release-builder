use std::sync::{ Arc, Mutex };
use std::thread::spawn;

pub fn multithread<F, I, O>(tasks: Vec<I>, num_threads: Option<usize>, task_fn: F) -> Vec<O>
where
    F: Fn(I) -> O + Send + Clone + 'static,
    I: Send + 'static,
    O: Send + 'static
{
    let num_tasks = tasks.len();
    let wrapped_tasks = Arc::new(Mutex::new(tasks.into_iter().enumerate()));
    let num_threads = num_threads.unwrap_or_else(num_cpus::get);
    let mut join_handles = Vec::with_capacity(num_threads);

    for _ in 0..num_threads {
        let wrapped_tasks = Arc::clone(&wrapped_tasks);
        let task_fn = task_fn.clone();

        let join_handle = spawn(move || {
            let mut results = vec![];

            loop {
                let mut unlocked_wrapped_tasks = wrapped_tasks.lock().unwrap();
                let task = unlocked_wrapped_tasks.next();

                // unlock as quickly as possible,
                // let other threads get at the data
                drop(unlocked_wrapped_tasks);

                match task {
                    Some((i, task)) => {
                        let res = task_fn(task);
                        results.push((i, res));
                    }
                    None => { break }
                }
            }

            results
        });

        join_handles.push(join_handle);
    }

    let mut thread_results = Vec::with_capacity(num_tasks);

    for thread in join_handles.into_iter() {
        let res = thread.join().unwrap();
        res.into_iter().for_each(|e| thread_results.push(e));
    }

    thread_results.sort_unstable_by_key(|e| e.0);
    thread_results.into_iter()
        .map(|e| e.1)
        .collect()
}
