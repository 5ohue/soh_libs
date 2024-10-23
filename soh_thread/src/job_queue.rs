//-----------------------------------------------------------------------------
use super::Job;
use std::collections::VecDeque as Queue;
//-----------------------------------------------------------------------------
/// This structure stores a queue with the jobs that are yet to be sent to
/// worker threads and it keeps track of how many jobs are currently being
/// executed.
pub struct JobQueue {
    jobs: Queue<Job>,
    in_process: usize,
}

impl JobQueue {
    /// Creates new empty queue
    pub const fn new() -> Self {
        JobQueue {
            jobs: Queue::new(),
            in_process: 0,
        }
    }

    /// Adds a new job to the end of the queue. This doesn't immediately start
    /// the job's execution.
    pub fn add_job<F>(&mut self, job_name: &'static str, job: F)
    where
        F: FnOnce() -> anyhow::Result<()> + Send + 'static,
    {
        self.jobs.push_back((job_name, Box::new(job)));
    }

    /// Gets the number of jobs that are waiting in the queue and the jobs that
    /// are currently being executed.
    pub fn get_num_of_jobs(&self) -> usize {
        return self.jobs.len() + self.in_process;
    }

    /// Returns the job at the front of the queue and removes it from the queue.
    pub fn take_job(&mut self) -> Option<Job> {
        let job = self.jobs.pop_front();
        if job.is_some() {
            self.in_process += 1;
        }
        return job;
    }

    fn signal_finished(&mut self) {
        assert_ne!(self.in_process, 0);
        self.in_process -= 1;
    }
}

impl Default for JobQueue {
    fn default() -> Self {
        Self::new()
    }
}

//-----------------------------------------------------------------------------
/// Global instance of a job queue
pub static JOB_QUEUE: std::sync::Mutex<JobQueue> = std::sync::Mutex::new(JobQueue::new());

/// Add a new job to the global job queue
#[macro_export]
macro_rules! add_job {
    ($job_name:expr, $lambda:expr) => {
        $crate::JOB_QUEUE
            .lock()
            .unwrap()
            .add_job($job_name, $lambda)
    };
}

//-----------------------------------------------------------------------------
/// This structure is used to automatically signal that a job has been finished
/// once it goes out of scope.
///
/// This helps to avoid having to call [JobQueue::signal_finished] manually
pub struct JobQueueHandle;

impl Drop for JobQueueHandle {
    fn drop(&mut self) {
        JOB_QUEUE.lock().unwrap().signal_finished();
    }
}

//-----------------------------------------------------------------------------
