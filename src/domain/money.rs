#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Money(u64);

impl Money {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn checked_add(self, other: Money) -> Option<Money> {
        self.0.checked_add(other.0).map(Self)
    }

    pub fn checked_mul(self, quantity: u32) -> Option<Money> {
        self.0.checked_mul(u64::from(quantity)).map(Self)
    }

    pub fn as_minor_units(&self) -> u64 {
        self.0
    }
}
