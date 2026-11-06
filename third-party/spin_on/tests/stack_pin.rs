//! Public-API regression for the compatible replacement of stack pinning.

use std::{cell::Cell, future::Future, marker::PhantomPinned, pin::Pin, rc::Rc};
use std::task::{Context, Poll};

struct AddressStable {
    address: Cell<*const Self>,
    polls: Cell<u8>,
    drops: Rc<Cell<usize>>,
    _pin: PhantomPinned,
}

impl Future for AddressStable {
    type Output = u8;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.as_ref().get_ref();
        let address = std::ptr::from_ref(this);
        if this.address.get().is_null() {
            this.address.set(address);
        } else {
            assert_eq!(this.address.get(), address);
        }
        let polls = this.polls.get();
        if polls == 3 {
            Poll::Ready(polls)
        } else {
            this.polls.set(polls + 1);
            context.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

impl Drop for AddressStable {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

#[test]
fn non_unpin_future_keeps_its_address_and_drops_once_after_pending_polls() {
    let drops = Rc::new(Cell::new(0));
    let future = AddressStable {
        address: Cell::new(std::ptr::null()),
        polls: Cell::new(0),
        drops: Rc::clone(&drops),
        _pin: PhantomPinned,
    };
    assert_eq!(spin_on::spin_on(future), 3);
    assert_eq!(drops.get(), 1);
}
