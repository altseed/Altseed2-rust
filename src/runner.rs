use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

// https://keens.github.io/blog/2019/07/07/rustnofuturetosonorunnerwotsukuttemita/

#[derive(Debug, Clone)]
struct SpinWaker;

static SPIN_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    SpinWaker::unsafe_clone,
    SpinWaker::unsafe_wake,
    SpinWaker::unsafe_wake_by_ref,
    SpinWaker::unsafe_drop,
);

impl SpinWaker {
    fn waker() -> Waker {
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

type PinsQueue = RefCell<VecDeque<Pin<Box<dyn Future<Output = ()>>>>>;

pub struct TaskRunner<'a> {
    context: RefCell<Context<'a>>,
    added_pins: PinsQueue,
    pins: PinsQueue,
    pins_tmp: PinsQueue,
}

impl<'a> TaskRunner<'a> {
    pub(crate) fn waker() -> std::task::Waker {
        SpinWaker::waker()
    }

    pub(crate) fn new(waker: &'a Waker) -> Self {
        TaskRunner {
            context: RefCell::new(Context::from_waker(waker)),
            added_pins: RefCell::new(VecDeque::new()),
            pins: RefCell::new(VecDeque::new()),
            pins_tmp: RefCell::new(VecDeque::new()),
        }
    }

    pub(crate) fn update(&self) {
        let mut pins = self.pins.borrow_mut();

        {
            let mut added = self.added_pins.borrow_mut();
            pins.append(&mut added);
            added.clear();
        }

        if pins.is_empty() {
            return;
        }

        let mut pins_tmp = self.pins_tmp.borrow_mut();

        let mut context = self.context.borrow_mut();
        while let Some(mut p) = pins.pop_back() {
            match p.as_mut().poll(&mut context) {
                Poll::Pending => pins_tmp.push_back(p),
                Poll::Ready(()) => (),
            }
        }

        std::mem::swap(&mut pins, &mut pins_tmp);
    }

    pub fn run<F>(&self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        let future = Pin::from(Box::new(future));
        self.run_pin(future)
    }

    pub fn run_pin<F>(&self, future: Pin<Box<F>>)
    where
        F: Future<Output = ()> + 'static,
    {
        self.added_pins.borrow_mut().push_back(future);
    }
}
