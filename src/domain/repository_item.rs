use uuid::Uuid;

pub trait RepositoryItem {
    type Id: RepositoryItemId;

    fn id(&self) -> &Self::Id;
}

pub trait RepositoryItemId {
    fn from_uuid(uuid: Uuid) -> Self;

    fn as_uuid(&self) -> &Uuid;
}
