use ordering_system::{Order, Pending};

fn reuse_moved_order(order: Order<Pending>) {
    let _confirmed = order.confirm();

    let _cancelled = order.cancel();
}

fn main() {}
