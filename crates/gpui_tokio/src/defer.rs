/* Taken from https://github.com/zed-industries/zed/blob/main/crates/util/src/util.rs
    APACHE LICENSE 2.0
*/

pub struct Deferred<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Drop for Deferred<F> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f()
        }
    }
}

/// Run the given function when the returned value is dropped (unless it's cancelled).
#[must_use]
pub fn defer<F: FnOnce()>(f: F) -> Deferred<F> {
    Deferred(Some(f))
}
