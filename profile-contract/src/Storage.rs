use multiversx_sc::mappers::{MapMapper, SingleValueMapper};
use crate::StorageMapper;
use multiversx_sc::types::ManagedAddress;


use crate::Project::ProjectModel;
use crate::User::UserModel;



#[multiversx_sc::contract]
pub trait ProfileContract {
    #[storage_mapper("users")]
    fn users(&self) -> MapMapper<ManagedAddress<Self::Api>, UserModel<Self::Api>>;

    #[storage_mapper("projects")]
    fn projects(&self) -> MapMapper<u32, ProjectModel<Self::Api>>;

    #[storage_mapper("project_id_counter")]
    fn project_id_counter(&self) -> SingleValueMapper<u32>;
}
