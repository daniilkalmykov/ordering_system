#[derive(Debug, PartialEq, Eq)]
pub enum StockError {
    InsufficientStock,
    InsufficientReserved,
    NoneQuantity,
    Overflow,
}
