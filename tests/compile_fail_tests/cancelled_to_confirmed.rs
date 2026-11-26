use ordering_system::{Cancelled, Order};

fn invalid_transition(order: Order<Cancelled>) {
    let _ = order.confirm();
}

fn main() {}
