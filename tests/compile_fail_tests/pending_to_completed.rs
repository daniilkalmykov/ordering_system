use ordering_system::{Order, Pending};

fn invalid_transition(order: Order<Pending>) {
    let _ = order.complete();
}

fn main() {}
