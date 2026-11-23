use uuid::Uuid;

use crate::{RepositoryItem, RepositoryItemId};

use super::OrderItem;

pub trait Order: RepositoryItem {
    fn items(&self) -> &[OrderItem];
    fn status(&self) -> &OrderStatus;
}

pub struct OrderRepositoryItem {
    id: OrderId,
    items: Vec<OrderItem>,
    status: OrderStatus,
}

impl OrderRepositoryItem {
    pub fn new(id: OrderId, items: Vec<OrderItem>) -> Self {
        Self {
            id,
            items,
            status: OrderStatus::Pending,
        }
    }
}

impl RepositoryItem for OrderRepositoryItem {
    type Id = OrderId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

pub enum OrderStatus {
    Pending,
    Confirmed,
    Completed,
    Cancelled,
}

pub struct OrderId(Uuid);

impl OrderId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl RepositoryItemId for OrderId {
    fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}
