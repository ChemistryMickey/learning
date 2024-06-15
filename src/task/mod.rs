#![allow(clippy::needless_return)]

pub mod simple_executor;

use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}
impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        return Task {
            future: Box::pin(future),
        };
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        return self.future.as_mut().poll(context);
    }
}