use uuid::Uuid;

use crate::Money;

pub struct Product {
    id: ProductId,
    name: String,
    price: Money,
    is_active: bool,
}

impl Product {
    pub fn new(id: ProductId, name: &str, price: Money) -> Self {
        Self {
            id,
            name: String::from(name),
            price,
            is_active: true,
        }
    }

    pub fn id(&self) -> &ProductId {
        &self.id
    }

    pub fn price(&self) -> &Money {
        &self.price
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProductId(Uuid);

impl ProductId {
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
