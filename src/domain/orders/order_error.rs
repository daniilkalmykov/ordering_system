#[derive(Debug, PartialEq, Eq)]
pub enum OrderError {
    ZeroItemsPicked,
    ZeroQuantity,
    DuplicateItems,
    MaxMoneyReached,
}
