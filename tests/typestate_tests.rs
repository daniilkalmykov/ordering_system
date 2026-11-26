use ordering_system::{
    Cancelled, Completed, Confirmed, Money, Order, OrderId, OrderItem, ProductId,
};

#[test]
fn pending_to_confirmed() {
    let order_id = OrderId::new();
    let order_items = vec![
        OrderItem::new(ProductId::new(), 1, Money::new(1)).unwrap(),
        OrderItem::new(ProductId::new(), 2, Money::new(2)).unwrap(),
        OrderItem::new(ProductId::new(), 3, Money::new(3)).unwrap(),
    ];

    let result = Order::new(order_id, order_items);

    assert!(result.is_ok());

    let _order: Order<Confirmed> = result.unwrap().confirm();
}

#[test]
fn pending_to_cancelled() {
    let order_id = OrderId::new();
    let order_items = vec![
        OrderItem::new(ProductId::new(), 1, Money::new(1)).unwrap(),
        OrderItem::new(ProductId::new(), 2, Money::new(2)).unwrap(),
        OrderItem::new(ProductId::new(), 3, Money::new(3)).unwrap(),
    ];

    let result = Order::new(order_id, order_items);

    assert!(result.is_ok());

    let _order: Order<Cancelled> = result.unwrap().cancel();
}

#[test]
fn confirmed_to_completed() {
    let order_id = OrderId::new();
    let order_items = vec![
        OrderItem::new(ProductId::new(), 1, Money::new(1)).unwrap(),
        OrderItem::new(ProductId::new(), 2, Money::new(2)).unwrap(),
        OrderItem::new(ProductId::new(), 3, Money::new(3)).unwrap(),
    ];

    let result = Order::new(order_id, order_items);

    assert!(result.is_ok());

    let _order: Order<Completed> = result.unwrap().confirm().complete();
}

#[test]
fn confirmed_to_cancelled() {
    let order_id = OrderId::new();
    let order_items = vec![
        OrderItem::new(ProductId::new(), 1, Money::new(1)).unwrap(),
        OrderItem::new(ProductId::new(), 2, Money::new(2)).unwrap(),
        OrderItem::new(ProductId::new(), 3, Money::new(3)).unwrap(),
    ];

    let result = Order::new(order_id, order_items);

    assert!(result.is_ok());

    let _order: Order<Cancelled> = result.unwrap().confirm().cancel();
}

#[test]
fn transitions_preserve_id() {
    let order_id = OrderId::new();
    let order_items = vec![
        OrderItem::new(ProductId::new(), 1, Money::new(1)).unwrap(),
        OrderItem::new(ProductId::new(), 2, Money::new(2)).unwrap(),
        OrderItem::new(ProductId::new(), 3, Money::new(3)).unwrap(),
    ];

    let result = Order::new(order_id, order_items);

    assert!(result.is_ok());

    let order = result.unwrap().confirm();
    assert_eq!(order.id(), &order_id);

    let order = order.complete();
    assert_eq!(order.id(), &order_id);
}

#[test]
fn transitions_preserve_items() {
    let order_id = OrderId::new();
    let order_items = vec![
        OrderItem::new(ProductId::new(), 1, Money::new(1)).unwrap(),
        OrderItem::new(ProductId::new(), 2, Money::new(2)).unwrap(),
        OrderItem::new(ProductId::new(), 3, Money::new(3)).unwrap(),
    ];

    let result = Order::new(order_id, order_items.clone());

    assert!(result.is_ok());

    let order = result.unwrap().confirm();
    assert_eq!(order.items(), &order_items);

    let order = order.cancel();
    assert_eq!(order.items(), &order_items);
}

#[test]
fn transitions_preserve_total_price() {
    let order_id = OrderId::new();
    let order_items = vec![
        OrderItem::new(ProductId::new(), 1, Money::new(1)).unwrap(),
        OrderItem::new(ProductId::new(), 2, Money::new(2)).unwrap(),
        OrderItem::new(ProductId::new(), 3, Money::new(3)).unwrap(),
    ];

    let result = Order::new(order_id, order_items);

    assert!(result.is_ok());

    let order = result.unwrap().confirm();
    assert_eq!(order.total_price(), Money::new(14));

    let order = order.cancel();
    assert_eq!(order.total_price(), Money::new(14));
}
