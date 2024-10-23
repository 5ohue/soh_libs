//-----------------------------------------------------------------------------
mod job_queue;
mod thread_pool;
mod worker;
//-----------------------------------------------------------------------------
use job_queue::JobQueueHandle;
use worker::Worker;
//-----------------------------------------------------------------------------
type Job = (
    &'static str,
    Box<dyn FnOnce() -> anyhow::Result<()> + Send + 'static>,
);
//-----------------------------------------------------------------------------
pub use job_queue::JobQueue;
pub use job_queue::JOB_QUEUE;
pub use thread_pool::ThreadPool;
//-----------------------------------------------------------------------------
