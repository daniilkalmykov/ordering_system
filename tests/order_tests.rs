use ordering_system::{Money, Order, OrderError, OrderId, OrderItem, Pending, ProductId};

#[test]
fn create_pending_order() {
    let order_id = OrderId::new();
    let order_items = vec![
        OrderItem::new(ProductId::new(), 1, Money::new(1)).unwrap(),
        OrderItem::new(ProductId::new(), 2, Money::new(2)).unwrap(),
        OrderItem::new(ProductId::new(), 3, Money::new(3)).unwrap(),
    ];

    let result = Order::new(order_id, order_items.clone());

    assert!(result.is_ok());
    assert_eq!(result, Order::<Pending>::new(order_id, order_items));
}

#[test]
fn empty_order_rejected() {
    let result = Order::new(OrderId::new(), vec![]);

    assert_eq!(result, Err(OrderError::ZeroItemsPicked));
}

#[test]
fn duplicate_products_rejected() {
    let order_id = OrderId::new();
    let product_id = ProductId::new();

    let order_items = vec![
        OrderItem::new(product_id, 1, Money::new(1)).unwrap(),
        OrderItem::new(product_id, 3, Money::new(3)).unwrap(),
    ];

    let result = Order::new(order_id, order_items);

    assert_eq!(result, Err(OrderError::DuplicateItems));
}

#[test]
fn different_products_allowed() {
    let order_id = OrderId::new();
    let product_id_1 = ProductId::new();
    let product_id_2 = ProductId::new();

    let order_items = vec![
        OrderItem::new(product_id_1, 1, Money::new(1)).unwrap(),
        OrderItem::new(product_id_2, 3, Money::new(3)).unwrap(),
    ];

    let result = Order::new(order_id, order_items);

    assert!(result.is_ok());

    assert_eq!(result.unwrap().items().len(), 2);
}

#[test]
fn order_total_price() {
    let order_id = OrderId::new();
    let order_items = vec![
        OrderItem::new(ProductId::new(), 1, Money::new(1)).unwrap(),
        OrderItem::new(ProductId::new(), 2, Money::new(2)).unwrap(),
        OrderItem::new(ProductId::new(), 3, Money::new(3)).unwrap(),
    ];

    let result = Order::new(order_id, order_items);

    assert!(result.is_ok());

    assert_eq!(result.unwrap().total_price(), Money::new(14));
}

#[test]
fn item_overflow_rejected() {
    let order_id = OrderId::new();
    let order_items = vec![
        OrderItem::new(ProductId::new(), 2, Money::new(u64::MAX)).unwrap(),
        OrderItem::new(ProductId::new(), 3, Money::new(3)).unwrap(),
    ];

    let result = Order::new(order_id, order_items);

    assert_eq!(result, Err(OrderError::MaxMoneyReached));
}

#[test]
fn total_overflow_rejected() {
    let order_id = OrderId::new();
    let order_items = vec![
        OrderItem::new(ProductId::new(), 1, Money::new(u64::MAX / 2)).unwrap(),
        OrderItem::new(ProductId::new(), 1, Money::new(u64::MAX / 2)).unwrap(),
        OrderItem::new(ProductId::new(), 1, Money::new(10000)).unwrap(),
    ];

    let result = Order::new(order_id, order_items);

    assert_eq!(result, Err(OrderError::MaxMoneyReached));
}

#[test]
fn maximum_valid_total() {
    let order_id = OrderId::new();
    let order_items = vec![
        OrderItem::new(ProductId::new(), 1, Money::new(u64::MAX - 100)).unwrap(),
        OrderItem::new(ProductId::new(), 1, Money::new(100)).unwrap(),
    ];

    let result = Order::new(order_id, order_items);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().total_price(), Money::new(u64::MAX));
}

#[test]
fn order_id_preserved() {
    let order_id = OrderId::new();
    let order_items = vec![
        OrderItem::new(ProductId::new(), 1, Money::new(1)).unwrap(),
        OrderItem::new(ProductId::new(), 2, Money::new(2)).unwrap(),
        OrderItem::new(ProductId::new(), 3, Money::new(3)).unwrap(),
    ];

    let result = Order::new(order_id, order_items);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().id(), &order_id);
}

#[test]
fn order_items_preserved() {
    let order_id = OrderId::new();
    let order_items = vec![
        OrderItem::new(ProductId::new(), 1, Money::new(1)).unwrap(),
        OrderItem::new(ProductId::new(), 2, Money::new(2)).unwrap(),
        OrderItem::new(ProductId::new(), 3, Money::new(3)).unwrap(),
    ];

    let result = Order::new(order_id, order_items.clone());

    assert!(result.is_ok());
    assert_eq!(result.unwrap().items(), &order_items);
}
