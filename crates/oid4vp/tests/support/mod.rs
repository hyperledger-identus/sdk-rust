use std::{
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};

use identus_core::{ClockError, UnixTimestampMillis, WallClock};
use identus_oid4vp::{
    RequestObjectReplayFuture, RequestObjectReplayGuard, RequestObjectReplayInput,
    RequestObjectSecurityEvidence, RequestObjectSecurityPolicy, RequestObjectValidationLimits,
    VerifiedRequestObject,
};

pub const NOW_SECONDS: u64 = 1_735_689_600;
pub const ISSUED_AT_SECONDS: u64 = NOW_SECONDS - 120;

struct FixedClock;

impl WallClock for FixedClock {
    fn now(&self) -> Result<UnixTimestampMillis, ClockError> {
        Ok(UnixTimestampMillis::new(NOW_SECONDS * 1_000))
    }
}

struct AcceptReplay;

impl RequestObjectReplayGuard for AcceptReplay {
    fn accept<'a>(&'a self, _input: RequestObjectReplayInput<'a>) -> RequestObjectReplayFuture<'a> {
        Box::pin(async { Ok(()) })
    }
}

pub fn secure(request: VerifiedRequestObject) -> RequestObjectSecurityEvidence {
    let policy = RequestObjectSecurityPolicy::static_discovery(
        300,
        30,
        RequestObjectValidationLimits::default(),
    )
    .expect("fixed security policy");
    block_on(request.validate_security(&policy, &FixedClock, &AcceptReplay))
        .expect("security-validated request object")
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
