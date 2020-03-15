use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

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

use std::cell::RefCell;
use std::collections::VecDeque;

type PinsQueue<E> = VecDeque<Pin<Box<dyn Future<Output = Result<(), E>>>>>;

pub struct TaskRunner<'a, E> {
    context: Context<'a>,
    added_pins: PinsQueue<E>,
    pins: PinsQueue<E>,
    pins_tmp: PinsQueue<E>,
}

impl<'a, E> TaskRunner<'a, E> {
    pub(crate) fn new(waker: &'a Waker) -> Self {
        TaskRunner {
            context: Context::from_waker(waker),
            added_pins: VecDeque::new(),
            pins: VecDeque::new(),
            pins_tmp: VecDeque::new(),
        }
    }

    pub(crate) fn update(&mut self) -> Result<(), E> {
        self.pins.append(&mut self.added_pins);

        if self.pins.is_empty() {
            return Ok(());
        }

        while let Some(mut p) = self.pins.pop_back() {
            match p.as_mut().poll(&mut self.context) {
                Poll::Pending => self.pins_tmp.push_back(p),
                Poll::Ready(error) => {
                    std::mem::swap(&mut self.pins, &mut self.pins_tmp);
                    return error;
                }
            }
        }

        std::mem::swap(&mut self.pins, &mut self.pins_tmp);

        Ok(())
    }

    pub fn run<F>(&mut self, future: F)
    where
        F: Future<Output = Result<(), E>> + 'static,
    {
        let future = Pin::from(Box::new(future));
        self.run_pin(future)
    }

    pub fn run_pin<F>(&mut self, future: Pin<Box<F>>)
    where
        F: Future<Output = Result<(), E>> + 'static,
    {
        self.added_pins.push_back(future);
    }
}
