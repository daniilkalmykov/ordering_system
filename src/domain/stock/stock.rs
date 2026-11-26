use super::StockError;
use crate::ProductId;

pub struct Stock {
    product_id: ProductId,
    available: u32,
    reserved: u32,
}

impl Stock {
    pub fn new(product_id: ProductId, available: u32) -> Self {
        Self {
            product_id,
            available,
            reserved: 0,
        }
    }

    pub fn restore(
        product_id: ProductId,
        available: u32,
        reserved: u32,
    ) -> Result<Self, StockError> {
        let _total_amount = available
            .checked_add(reserved)
            .ok_or(StockError::Overflow)?;

        Ok(Self {
            product_id,
            available,
            reserved,
        })
    }

    pub fn reserve(&mut self, quantity: u32) -> Result<(), StockError> {
        if quantity == 0 {
            return Err(StockError::NoneQuantity);
        }

        if quantity > self.available {
            return Err(StockError::InsufficientStock);
        }

        let reserved = self
            .reserved
            .checked_add(quantity)
            .ok_or(StockError::Overflow)?;

        self.available -= quantity;
        self.reserved = reserved;

        Ok(())
    }

    pub fn release(&mut self, quantity: u32) -> Result<(), StockError> {
        if quantity == 0 {
            return Err(StockError::NoneQuantity);
        }

        if quantity > self.reserved {
            return Err(StockError::InsufficientReserved);
        }

        let available = self
            .available
            .checked_add(quantity)
            .ok_or(StockError::Overflow)?;

        self.available = available;
        self.reserved -= quantity;

        Ok(())
    }

    pub fn consume_reserved(&mut self, quantity: u32) -> Result<(), StockError> {
        if quantity == 0 {
            return Err(StockError::NoneQuantity);
        }

        if quantity > self.reserved {
            return Err(StockError::InsufficientReserved);
        }

        self.reserved -= quantity;

        Ok(())
    }

    pub fn available(&self) -> u32 {
        self.available
    }

    pub fn reserved(&self) -> u32 {
        self.reserved
    }

    pub fn product_id(&self) -> &ProductId {
        &self.product_id
    }
}
