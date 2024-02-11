use std::{time::{Duration, Instant}, thread::{JoinHandle, self}, sync::{Arc, atomic::{AtomicBool, Ordering}}};

use log::error;

///
/// If maximum test turation will be exceeded - the panics throwed
pub struct TestDuration {
    id: String,
    duration: Duration,
    exit: Arc<AtomicBool>,
}
///
/// 
impl TestDuration {
    ///
    /// If maximum test turation will be exceeded - exiting testing process with error code 70
    /// - parent: String - name of the parent entity
    /// - duration - maximum test duration
    pub fn new(parent: impl Into<String>, duration: Duration) -> Self {
        Self {
            id: format!("{}/TestDuration", parent.into()),
            duration,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// The countdown begins, exiting process with error code 70 if duration exceeded
    pub fn run(&self) -> Result<JoinHandle<()>, std::io::Error> {
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        let duration = self.duration.clone();
        thread::Builder::new().name(format!("{}.run", self_id)).spawn(move || {
            let timer = Instant::now();
            loop {
                if exit.load(Ordering::SeqCst) {
                    break;
                }
                thread::sleep(Duration::from_millis(100));
                if exit.load(Ordering::SeqCst) {
                    break;
                }
                if timer.elapsed() > duration {
                    error!("{}.run | Maximum test duration ({:?}) exceeded", self_id, duration);
                    std::process::exit(70);   // SOFTWARE: ExitCode = 70
                }

            }
        })
    }
    ///
    /// Normal completion, must be called before the duration has expired
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}