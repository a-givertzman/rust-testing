use std::sync::atomic::{AtomicUsize, Ordering};

///
/// Taerdown provides calling a code at the end of the test function 
///  - each - calling only for current test function
///  - once - called only after all test functions 
pub struct Teardown<'a> {
    count: &'a AtomicUsize,
    each: &'a dyn Fn() -> (),
    once: &'a dyn Fn() -> (),
}

impl Teardown<'_> {
    fn new<'a>(count: &'a AtomicUsize, each: &'a dyn Fn() -> (), once: &'a dyn Fn() -> ()) -> Teardown<'a> {
        Teardown {
            count,
            each,
            once,
        }
    }
}

impl Drop for Teardown<'_> {
    fn drop(&mut self) {
        let _ = &(self.each)();
        self.count.fetch_sub(1, Ordering::SeqCst);
        if self.count.load(Ordering::SeqCst) <= 0 {
            let _ = &(self.once)();
        }
    }
}
