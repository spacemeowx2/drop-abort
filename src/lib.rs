use futures_util::future::{self, AbortHandle, Abortable, Future};

#[derive(Debug)]
pub struct DropAbortHandle(AbortHandle);
impl Drop for DropAbortHandle {
    fn drop(&mut self) {
        self.0.abort();
    }
}

pub fn abortable<Fut>(fut: Fut) -> (Abortable<Fut>, DropAbortHandle)
where
    Fut: Future,
{
    let (fut, handle) = future::abortable(fut);
    (fut, DropAbortHandle(handle))
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::future::Aborted;

    #[test]
    fn test_drop_abort() {
        futures::executor::block_on(async {
            let (fut, handle) = abortable(async {});
            drop(handle);
            let r = fut.await;
            assert_eq!(r, Err(Aborted))
        });
    }
}
