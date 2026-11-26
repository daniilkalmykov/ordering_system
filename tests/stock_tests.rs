use ordering_system::{ProductId, Stock, StockError};

#[test]
fn create_stock() {
    let stock = Stock::new(ProductId::new(), 10);

    assert_eq!(stock.available(), 10);
    assert_eq!(stock.reserved(), 0);
}

#[test]
fn create_empty_stock() {
    let stock = Stock::new(ProductId::new(), 0);

    assert_eq!(stock.available(), 0);
    assert_eq!(stock.reserved(), 0);
}

#[test]
fn stock_product_id_preserved() {
    let product_id = ProductId::new();
    let stock = Stock::new(product_id, 10);

    assert_eq!(stock.product_id(), &product_id);
}

#[test]
fn restore_valid_stock() {
    let stock = Stock::restore(ProductId::new(), 7, 3);

    assert!(stock.is_ok());

    let stock = stock.unwrap();

    assert_eq!(stock.available(), 7);
    assert_eq!(stock.reserved(), 3);
}

#[test]
fn restore_overflow() {
    let stock = Stock::restore(ProductId::new(), u32::MAX, 1);

    assert_eq!(stock, Err(StockError::Overflow));
}

#[test]
fn restore_maximum_stock() {
    let stock = Stock::restore(ProductId::new(), u32::MAX - 1, 1);

    assert!(stock.is_ok());
}

#[test]
fn reserve_partial() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(3).is_ok());

    assert_eq!(stock.available(), 7);
    assert_eq!(stock.reserved(), 3);
}

#[test]
fn reserve_all() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(10).is_ok());

    assert_eq!(stock.available(), 0);
    assert_eq!(stock.reserved(), 10);
}

#[test]
fn reserve_twice() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(3).is_ok());
    assert!(stock.reserve(2).is_ok());

    assert_eq!(stock.available(), 5);
    assert_eq!(stock.reserved(), 5);
}

#[test]
fn reserve_insufficient_stock() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert_eq!(stock.reserve(11), Err(StockError::InsufficientStock));
}

#[test]
fn reserve_zero() {
    let mut stock = Stock::new(ProductId::new(), 10);
    assert_eq!(stock.reserve(0), Err(StockError::NoneQuantity));
}

#[test]
fn release_partial() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(3).is_ok());
    assert!(stock.release(2).is_ok());

    assert_eq!(stock.available(), 9);
    assert_eq!(stock.reserved(), 1);
}

#[test]
fn release_all() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(3).is_ok());
    assert!(stock.release(3).is_ok());

    assert_eq!(stock.available(), 10);
    assert_eq!(stock.reserved(), 0);
}

#[test]
fn release_twice() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(5).is_ok());

    assert!(stock.release(2).is_ok());
    assert!(stock.release(3).is_ok());

    assert_eq!(stock.available(), 10);
    assert_eq!(stock.reserved(), 0);
}

#[test]
fn release_insufficient_reserved() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(2).is_ok());

    assert_eq!(stock.release(6), Err(StockError::InsufficientReserved));
}

#[test]
fn release_zero() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(2).is_ok());

    assert_eq!(stock.release(0), Err(StockError::NoneQuantity));
}

#[test]
fn consume_partial() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(3).is_ok());
    assert!(stock.consume_reserved(2).is_ok());

    assert_eq!(stock.available(), 7);
    assert_eq!(stock.reserved(), 1);
}

#[test]
fn consume_all_reserved() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(3).is_ok());
    assert!(stock.consume_reserved(3).is_ok());

    assert_eq!(stock.available(), 7);
    assert_eq!(stock.reserved(), 0);
}

#[test]
fn consume_when_available_zero() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(10).is_ok());
    assert!(stock.consume_reserved(10).is_ok());

    assert_eq!(stock.available(), 0);
    assert_eq!(stock.reserved(), 0);
}

#[test]
fn consume_insufficient_reserved() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(2).is_ok());

    assert_eq!(
        stock.consume_reserved(6),
        Err(StockError::InsufficientReserved)
    );
}

#[test]
fn consume_zero() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(2).is_ok());

    assert_eq!(stock.consume_reserved(0), Err(StockError::NoneQuantity));
}

#[test]
fn reserve_preserves_total() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(3).is_ok());

    assert_eq!(stock.available() + stock.reserved(), 10);
}

#[test]
fn consume_decreases_total() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(3).is_ok());
    assert!(stock.consume_reserved(2).is_ok());

    assert_eq!(stock.available() + stock.reserved() + 2, 10);
}

#[test]
fn failed_operations_preserve_state() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(11).is_err());
    assert_eq!(stock.available(), 10);
    assert_eq!(stock.reserved(), 0);

    assert!(stock.release(5).is_err());
    assert_eq!(stock.available(), 10);
    assert_eq!(stock.reserved(), 0);

    assert!(stock.consume_reserved(5).is_err());
    assert_eq!(stock.available(), 10);
    assert_eq!(stock.reserved(), 0);
}

#[test]
fn maximum_quantity_operations() {
    let mut stock = Stock::new(ProductId::new(), u32::MAX);

    assert!(stock.reserve(10).is_ok());
    assert!(stock.release(9).is_ok());
    assert_eq!(stock.available(), u32::MAX - 1);
    assert_eq!(stock.reserved(), 1);

    assert!(stock.release(1).is_ok());

    assert!(stock.reserve(u32::MAX).is_ok());
    assert_eq!(stock.available(), 0);
    assert_eq!(stock.reserved(), u32::MAX);

    assert!(stock.release(u32::MAX).is_ok());

    assert_eq!(stock.available(), u32::MAX);
    assert_eq!(stock.reserved(), 0);
}

#[test]
fn full_stock_lifecycle() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(4).is_ok());
    assert!(stock.release(2).is_ok());
    assert!(stock.consume_reserved(2).is_ok());

    assert_eq!(stock.available(), 8);
    assert_eq!(stock.reserved(), 0);
}

#[test]
fn release_preserves_total() {
    let mut stock = Stock::new(ProductId::new(), 10);

    assert!(stock.reserve(3).is_ok());
    assert!(stock.release(2).is_ok());

    assert_eq!(stock.available() + stock.reserved(), 10);
}
