use std::num::{NonZeroU32, NonZeroU64};

use crate::ProductId;

pub struct OrderItem {
    product_id: ProductId,
    quantity: NonZeroU32,
    unit_price: NonZeroU64,
}

impl OrderItem {
    pub fn new(product_id: ProductId, quantity: u32, unit_price: u64) -> Self {
        Self {
            product_id,
            quantity: NonZeroU32::new(quantity).unwrap(),
            unit_price: NonZeroU64::new(unit_price).unwrap(),
        }
    }
}
