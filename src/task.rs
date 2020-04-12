use std::task::{RawWaker, RawWakerVTable, Waker};

// https://keens.github.io/blog/2019/07/07/rustnofuturetosonorunnerwotsukuttemita/

#[derive(Debug, Clone)]
pub(crate) struct SpinWaker;

static SPIN_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    SpinWaker::unsafe_clone,
    SpinWaker::unsafe_wake,
    SpinWaker::unsafe_wake_by_ref,
    SpinWaker::unsafe_drop,
);

impl SpinWaker {
    pub(crate) fn waker() -> Waker {
        unsafe { Waker::from_raw(Self::new().into_raw_waker()) }
    }

    fn new() -> Self {
        Self
    }

    unsafe fn into_raw_waker(self) -> RawWaker {
        let ptr = Box::into_raw(Box::new(self)) as *const ();
        RawWaker::new(ptr, &SPIN_WAKER_VTABLE)
    }

    unsafe fn unsafe_clone(this: *const ()) -> RawWaker {
        let ptr = this as *const Self;
        Box::new(ptr.as_ref().unwrap().clone()).into_raw_waker()
    }

    fn wake(self: Self) {}

    unsafe fn unsafe_wake(this: *const ()) {
        let ptr = this as *mut Self;
        Box::from_raw(ptr).wake()
    }

    fn wake_by_ref(&self) {
        Box::new(self.clone()).wake()
    }

    unsafe fn unsafe_wake_by_ref(this: *const ()) {
        let ptr = this as *const Self;
        ptr.as_ref().unwrap().wake_by_ref()
    }

    unsafe fn unsafe_drop(this: *const ()) {
        let ptr = this as *mut Self;
        Box::from_raw(ptr);
    }
}

use crate::{engine::Engine, error::AltseedResult};

pub enum Cont {
    Fin,
    Then(Box<dyn FnOnce(&mut Engine) -> AltseedResult<()>>),
}

impl Cont {
    pub fn fin<E>() -> Result<Cont, E> {
        Ok(Cont::Fin)
    }

    #[inline]
    pub fn then<F: FnOnce(&mut Engine) -> AltseedResult<()> + 'static, E>(f: F) -> Result<Cont, E> {
        Ok(Cont::Then(Box::new(f)))
    }
}
