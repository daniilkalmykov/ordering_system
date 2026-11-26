use ordering_system::{Order, Pending};

fn invalid_transition(order: Order<Pending>) {
    let order: Order<Pending> = order.confirm();
}

fn main() {}
