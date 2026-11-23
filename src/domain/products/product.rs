use std::num::NonZeroU64;

use uuid::Uuid;

use crate::{RepositoryItem, RepositoryItemId};

pub trait Product: RepositoryItem {
    fn price(&self) -> u64;
    fn name(&self) -> &str;
    fn is_active(&self) -> bool;
}

pub struct ProductRepositoryItem {
    id: ProductId,
    name: String,
    price: NonZeroU64,
    is_active: bool,
}

impl ProductRepositoryItem {
    pub fn new(id: ProductId, name: &str, price: u64) -> Self {
        Self {
            id,
            name: String::from(name),
            price: NonZeroU64::new(price).unwrap(),
            is_active: true,
        }
    }
}

impl RepositoryItem for ProductRepositoryItem {
    type Id = ProductId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Product for ProductRepositoryItem {
    fn price(&self) -> u64 {
        self.price.get()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}

pub struct ProductId(Uuid);

impl ProductId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl RepositoryItemId for ProductId {
    fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}
