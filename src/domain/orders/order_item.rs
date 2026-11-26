use std::num::NonZeroU32;

use super::OrderError;
use crate::{Money, ProductId};

#[derive(Debug, PartialEq, Clone)]
pub struct OrderItem {
    product_id: ProductId,
    quantity: NonZeroU32,
    unit_price: Money,
}

impl OrderItem {
    pub fn new(
        product_id: ProductId,
        quantity: u32,
        unit_price: Money,
    ) -> Result<Self, OrderError> {
        let quantity = match NonZeroU32::new(quantity) {
            Some(quantity) => quantity,
            None => return Err(OrderError::ZeroQuantity),
        };

        Ok(Self {
            product_id,
            quantity,
            unit_price,
        })
    }

    pub fn id(&self) -> &ProductId {
        &self.product_id
    }

    pub fn quantity(&self) -> u32 {
        self.quantity.get()
    }

    pub fn unit_price(&self) -> Money {
        self.unit_price
    }

    pub fn total_price(&self) -> Option<Money> {
        self.unit_price.checked_mul(self.quantity.get())
    }
}
