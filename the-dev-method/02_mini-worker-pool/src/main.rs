use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Job {
    id: u32,
}

// shared type
type Queue = Arc<Mutex<VecDeque<Job>>>;

fn main() {
    let workers = 4;
    let total_jobs = 20;

    // --------------------------
    // create shared queue
    // --------------------------
    let queue: Queue = Arc::new(Mutex::new(VecDeque::new()));

    // push jobs into queue
    {
        let mut q = queue.lock().unwrap();
        for i in 0..total_jobs {
            q.push_back(Job { id: i });
        }
    }

    println!("Added {} jobs\n", total_jobs);

    let start = Instant::now();

    let mut handles = vec![];

    // --------------------------
    // spawn workers
    // --------------------------
    for worker_id in 0..workers {
        let queue = Arc::clone(&queue);

        let handle = thread::spawn(move || {
            loop {
                // 🔒 lock only to take job
                let job = {
                    let mut q = queue.lock().unwrap();
                    q.pop_front()
                }; // 🔓 unlock happens here

                match job {
                    Some(job) => {
                        println!("Worker {} processing job {}", worker_id, job.id);

                        // simulate heavy work
                        thread::sleep(Duration::from_millis(200));
                    }
                    None => {
                        println!("Worker {} done (no jobs left)", worker_id);
                        break;
                    }
                }
            }
        });

        handles.push(handle);
    }

    // wait all workers
    for h in handles {
        h.join().unwrap();
    }

    let duration = start.elapsed();

    println!("\nAll jobs finished");
    println!("Time taken: {:?}", duration);
}

/*

What this program does (very simple)

It will:

Create 100 jobs

Put them into a shared queue

Spawn 4 worker threads

Workers pick jobs one by one

Process them

Exit when queue is empty

You will SEE threads working in parallel.


--------

logs:
Added 20 jobs

Worker 0 processing job 0
Worker 2 processing job 2
Worker 1 processing job 1
Worker 3 processing job 3
Worker 1 processing job 4
Worker 0 processing job 5
Worker 3 processing job 6
Worker 2 processing job 7
Worker 3 processing job 8
Worker 2 processing job 10
Worker 1 processing job 9
Worker 0 processing job 11
Worker 3 processing job 12
Worker 1 processing job 13
Worker 2 processing job 14
Worker 0 processing job 15
Worker 3 processing job 16
Worker 1 processing job 17
Worker 2 processing job 18
Worker 0 processing job 19
Worker 2 done (no jobs left)
Worker 0 done (no jobs left)
Worker 1 done (no jobs left)
Worker 3 done (no jobs left)

All jobs finished
Time taken: 1.017296292s
*/
