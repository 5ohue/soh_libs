//-----------------------------------------------------------------------------
use super::{Job, Worker};
use std::sync::{mpsc, Arc, Mutex};
//-----------------------------------------------------------------------------

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Creates a new thread pool with specified number of worker threads.
    ///
    /// The number of worker threads has to be greater than 2!
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 2);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for _ in 1..size {
            workers.push(Worker::new(receiver.clone()));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// This functions checks if there are any jobs in the queue. If there are
    /// jobs that are yet to be submitted to the worker threads it sends it to
    /// them.
    ///
    /// It returns true if the thread pool has finished executing all of the
    /// jobs, otherwise it returns false.
    pub fn poke(&self) -> bool {
        let queue = &mut super::JOB_QUEUE.lock().unwrap();

        if queue.get_num_of_jobs() == 0 {
            return true;
        }

        /*
         * There may or may not be a job in the queue.
         *
         * If there is, send it to worker threads.
         * If there isn't just return `false` because there is a job that's
         * currently being executed.
         */
        if let Some(job) = queue.take_job() {
            self.sender.as_ref().unwrap().send(job).unwrap();
        }

        return false;
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

//-----------------------------------------------------------------------------
