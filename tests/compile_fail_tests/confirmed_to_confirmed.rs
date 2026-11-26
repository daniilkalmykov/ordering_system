use ordering_system::{Confirmed, Order};

fn invalid_transition(order: Order<Confirmed>) {
    let _ = order.confirm();
}

fn main() {}
