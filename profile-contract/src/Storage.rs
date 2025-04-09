/* use crate::{UserImpl, Project};
use multiversx_sc::storage::mappers::{SingleValueMapper, MapMapper};
use multiversx_sc::types::{ManagedAddress};
use multiversx_sc::api::StorageMapperApi;

pub trait StorageModuleImpl<M: StorageMapperApi>: StorageModule {
    fn users(&self) -> MapMapper<ManagedAddress<M>, UserImpl<M>> {
        MapMapper::new(self, b"users")
    }

    fn projects(&self) -> MapMapper<u32, Project<M>> {
        MapMapper::new(self, b"projects")
    }

    fn project_id_counter(&self) -> SingleValueMapper<u32> {
        SingleValueMapper::new(self, b"project_id_counter")
    }
}
 */