use std::{thread::JoinHandle, any::Any};

use log::{info, error};

///
/// Performs JoinHandle.join() with some debug output
pub trait WaitTread {
    fn wait(self) -> Result<(), Box<dyn Any + Send>>;
}
///
/// 
impl WaitTread for JoinHandle<()> {
    ///
    /// Performs JoinHandle.join() some debug output
    fn wait(self) -> Result<(), Box<dyn Any + Send>> {
        let thread_id = format!("{:?}-{:?}", self.thread().id(), self.thread().name());
        info!("Waiting for thread: {:?}...", thread_id);
        let r = self.join();
        match &r {
            Ok(_) => {
                info!("Waiting for thread: '{}' - finished", thread_id);
            },
            Err(err) => {
                error!("Waiting for thread '{}' error: {:?}", thread_id, err);                
            },
        }
        r
    }
}
