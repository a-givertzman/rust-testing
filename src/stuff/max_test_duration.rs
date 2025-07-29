use std::{time::{Duration, Instant}, thread::{JoinHandle, self}, sync::{Arc, atomic::{AtomicBool, Ordering}}};

///
/// If maximum test turation will be exceeded - the panics throwed
pub struct TestDuration {
    dbg: String,
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
            dbg: format!("{}/TestDuration", parent.into()),
            duration,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// The countdown begins, exiting process with error code 70 if duration exceeded
    pub fn run(&self) -> Result<JoinHandle<()>, std::io::Error> {
        let dbg = self.dbg.clone();
        let dbg_cln = self.dbg.clone();
        let exit = self.exit.clone();
        let duration = self.duration.clone();
        let orig_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let dbg = &dbg_cln;
            // invoke the default handler and exit the process
            orig_hook(panic_info);
            if let Some(location) = panic_info.location() {
                if location.file().ends_with("max_test_duration.rs") {
                    panic!("{dbg}.run | Terminated");
                }
            }
        }));
        thread::Builder::new().name(format!("{}.run", dbg)).spawn(move || {
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
                    log::error!("{}.run | Maximum test duration ({:?}) exceeded", dbg, duration);
                    panic!("{}.run | Terminating...", dbg);
                    // std::process::exit(70);   // SOFTWARE: ExitCode = 70
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