//-----------------------------------------------------------------------------
use super::Job;
use std::sync::mpsc;
//-----------------------------------------------------------------------------

pub struct Worker {
    pub thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(receiver: std::sync::Arc<std::sync::Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = std::thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok((job_name, job)) => {
                    let _handle = super::JobQueueHandle;

                    if let Err(e) = job() {
                        #[cfg(feature = "log")]
                        soh_log::log_warning!(
                            "Error occured when running the task \"{}\":\n{}",
                            job_name,
                            e
                        );

                        #[cfg(not(feature = "log"))]
                        eprintln!(
                            "Error occured when running the task \"{}\":\n{}",
                            job_name, e
                        );
                    }
                }
                Err(_) => {
                    break;
                }
            }
        });

        Worker {
            thread: Some(thread),
        }
    }
}

//-----------------------------------------------------------------------------
