use ordering_system::{Completed, Order};

fn invalid_transition(order: Order<Completed>) {
    let _ = order.cancel();
}

fn main() {}
