use std::collections::HashSet;

use ordering_system::{Money, OrderId, Product, ProductId};
use uuid::Uuid;

#[test]
fn create_product() {
    let id = ProductId::new();
    let name = "Product 1";
    let price = Money::new(10);

    let product = Product::new(id, name, price);

    assert_eq!(product.id(), &id);
    assert_eq!(product.name(), name);
    assert_eq!(product.price(), &price);
}

#[test]
fn product_active_by_default() {
    let product = Product::new(ProductId::new(), "Product 1", Money::new(10));

    assert!(product.is_active())
}

#[test]
fn product_id_uuid_roundtrip() {
    let product_id = ProductId::new();
    let product_id_from_uuid = ProductId::from_uuid(*product_id.as_uuid());

    assert_eq!(product_id, product_id_from_uuid);
}

#[test]
fn order_id_uuid_roundtrip() {
    let order_id = OrderId::new();
    let order_id_from_uuid = OrderId::from_uuid(*order_id.as_uuid());

    assert_eq!(order_id, order_id_from_uuid);
}

#[test]
fn product_id_equality() {
    let uuid = Uuid::new_v4();

    assert_eq!(ProductId::from_uuid(uuid), ProductId::from_uuid(uuid));
}

#[test]
fn product_id_hash() {
    let uuid = Uuid::new_v4();

    let product_id_1 = ProductId::from_uuid(uuid);
    let product_id_2 = ProductId::from_uuid(uuid);

    let mut hash_set = HashSet::new();

    assert!(hash_set.insert(product_id_1));
    assert!(!hash_set.insert(product_id_2));
}

#[test]
fn zero_price_product() {
    let id = ProductId::new();
    let name = "Zero price product";
    let price = Money::new(0);

    let product = Product::new(id, name, price);

    assert_eq!(product.id(), &id);
    assert_eq!(product.name(), name);
    assert_eq!(product.price(), &price);
    assert!(product.is_active())
}
