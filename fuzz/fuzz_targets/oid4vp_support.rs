use std::{future::Future, pin::pin, task::{Context, Poll, Waker}};

use identus_core::{ClockError, UnixTimestampMillis, WallClock};
use identus_oid4vp::{
    RequestObjectReplayFuture, RequestObjectReplayGuard, RequestObjectReplayInput,
    RequestObjectSecurityEvidence, RequestObjectSecurityPolicy, RequestObjectValidationLimits,
    VerifiedRequestObject,
};

pub const NOW_SECONDS: u64 = 1_735_689_600;

struct Clock;
impl WallClock for Clock {
    fn now(&self) -> Result<UnixTimestampMillis, ClockError> {
        Ok(UnixTimestampMillis::new(NOW_SECONDS * 1_000))
    }
}

struct Replay;
impl RequestObjectReplayGuard for Replay {
    fn accept<'a>(
        &'a self,
        _input: RequestObjectReplayInput<'a>,
    ) -> RequestObjectReplayFuture<'a> {
        Box::pin(async { Ok(()) })
    }
}

pub fn secure(request: VerifiedRequestObject) -> Option<RequestObjectSecurityEvidence> {
    let policy = RequestObjectSecurityPolicy::static_discovery(
        300,
        30,
        RequestObjectValidationLimits::default(),
    )
    .ok()?;
    block_on(request.validate_security(&policy, &Clock, &Replay)).ok()
}

fn block_on<F: Future>(future: F) -> F::Output {
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    let mut future = pin!(future);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}
