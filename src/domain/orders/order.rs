use super::{OrderError, OrderItem};
use crate::Money;
use std::collections::HashSet;
use std::marker::PhantomData;
use uuid::Uuid;

mod sealed {
    pub trait Sealed {}
}

pub trait Status: sealed::Sealed {}

#[derive(Debug, PartialEq)]
pub struct Pending;

impl sealed::Sealed for Pending {}
impl Status for Pending {}

pub struct Confirmed;

impl sealed::Sealed for Confirmed {}
impl Status for Confirmed {}

pub struct Completed;

impl sealed::Sealed for Completed {}
impl Status for Completed {}

pub struct Cancelled;

impl sealed::Sealed for Cancelled {}
impl Status for Cancelled {}

#[derive(Debug, PartialEq)]
pub struct Order<State: Status = Pending> {
    id: OrderId,
    items: Vec<OrderItem>,
    _state: PhantomData<State>,
    total_price: Money,
}

impl Order<Pending> {
    pub fn new(id: OrderId, items: Vec<OrderItem>) -> Result<Self, OrderError> {
        if items.is_empty() {
            return Err(OrderError::ZeroItemsPicked);
        }

        let mut seen_ids = HashSet::with_capacity(items.len());

        for item in &items {
            if !seen_ids.insert(item.id()) {
                return Err(OrderError::DuplicateItems);
            }
        }

        let total_price = items.iter().try_fold(Money::new(0), |total, item| {
            let price = item.total_price().ok_or(OrderError::MaxMoneyReached)?;

            total.checked_add(price).ok_or(OrderError::MaxMoneyReached)
        })?;

        Ok(Self {
            id,
            items,
            _state: PhantomData,
            total_price,
        })
    }

    pub fn confirm(self) -> Order<Confirmed> {
        Order {
            id: self.id,
            items: self.items,
            _state: PhantomData,
            total_price: self.total_price,
        }
    }

    pub fn cancel(self) -> Order<Cancelled> {
        Order {
            id: self.id,
            items: self.items,
            _state: PhantomData,
            total_price: self.total_price,
        }
    }
}

impl Order<Confirmed> {
    pub fn complete(self) -> Order<Completed> {
        Order {
            id: self.id,
            items: self.items,
            _state: PhantomData,
            total_price: self.total_price,
        }
    }

    pub fn cancel(self) -> Order<Cancelled> {
        Order {
            id: self.id,
            items: self.items,
            _state: PhantomData,
            total_price: self.total_price,
        }
    }
}

impl<State: Status> Order<State> {
    pub fn id(&self) -> &OrderId {
        &self.id
    }

    pub fn items(&self) -> &[OrderItem] {
        &self.items
    }

    pub fn total_price(&self) -> Money {
        self.total_price
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(Uuid);

impl OrderId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for OrderId {
    fn default() -> Self {
        Self::new()
    }
}
