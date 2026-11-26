use ordering_system::{Money, OrderError, OrderItem, ProductId};

#[test]
fn create_order_item() {
    let product_id = ProductId::new();
    let price = Money::new(10);
    let quantity = 10;

    let order_item = OrderItem::new(product_id, quantity, price);

    assert!(order_item.is_ok());
    assert_eq!(
        order_item,
        Ok(OrderItem::new(product_id, quantity, price).unwrap())
    );

    let order_item = order_item.unwrap();

    assert_eq!(order_item.id(), &product_id);
    assert_eq!(order_item.quantity(), quantity);
    assert_eq!(order_item.unit_price(), price);
    assert_eq!(order_item.total_price(), Some(Money::new(100)));
}

#[test]
fn zero_quantity_rejected() {
    let product_id = ProductId::new();
    let price = Money::new(10);
    let quantity = 0;

    let order_item = OrderItem::new(product_id, quantity, price);

    assert_eq!(order_item, Err(OrderError::ZeroQuantity));
}

#[test]
fn item_product_id_preserved() {
    let product_id = ProductId::new();
    let price = Money::new(10);
    let quantity = 10;

    let order_item = OrderItem::new(product_id, quantity, price);

    assert!(order_item.is_ok());

    let order_item = order_item.unwrap();

    assert_eq!(order_item.id(), &product_id);
}

#[test]
fn item_quantity_preserved() {
    let product_id = ProductId::new();
    let price = Money::new(10);
    let quantity = 10;

    let order_item = OrderItem::new(product_id, quantity, price);

    assert!(order_item.is_ok());

    let order_item = order_item.unwrap();

    assert_eq!(order_item.quantity(), quantity);
}

#[test]
fn item_unit_price_preserved() {
    let product_id = ProductId::new();
    let price = Money::new(10);
    let quantity = 10;

    let order_item = OrderItem::new(product_id, quantity, price);

    assert!(order_item.is_ok());

    let order_item = order_item.unwrap();

    assert_eq!(order_item.unit_price(), price);
}

#[test]
fn item_total_price() {
    let product_id = ProductId::new();
    let price = Money::new(10);
    let quantity = 10;

    let order_item = OrderItem::new(product_id, quantity, price);

    assert!(order_item.is_ok());

    let order_item = order_item.unwrap();

    assert_eq!(order_item.total_price(), Some(Money::new(100)));
}

#[test]
fn item_price_overflow() {
    let product_id = ProductId::new();
    let price = Money::new(u64::MAX);
    let quantity = 10;

    let order_item = OrderItem::new(product_id, quantity, price);

    assert!(order_item.is_ok());

    let order_item = order_item.unwrap();

    assert_eq!(order_item.total_price(), None);
}

#[test]
fn zero_price_item() {
    let product_id = ProductId::new();
    let price = Money::new(0);
    let quantity = 10;

    let order_item = OrderItem::new(product_id, quantity, price);

    assert!(order_item.is_ok());

    let order_item = order_item.unwrap();

    assert_eq!(order_item.total_price(), Some(Money::new(0)));
}
