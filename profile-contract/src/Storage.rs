use multiversx_sc::types::{ManagedAddress, BigUint};
use multiversx_sc::storage::{Mapping, SingleValue};
use multiversx_sc::api::BlockchainApi;

use crate::user_model::UserProfile;
use crate::project_model::Project;

#[multiversx_sc::module]
pub trait StorageModule: BlockchainApi {

    #[view(getUserProfile)]
    #[storage_mapper("user_profile")]
    fn user_profile(&self, address: &ManagedAddress<Self::Api>) -> Mapping<ManagedAddress<Self::Api>, UserProfile<Self::Api>>;

    #[view(getNextProjectId)]
    #[storage_mapper("next_project_id")]
    fn next_project_id(&self) -> SingleValue<u64>;

    #[view(getProject)]
    #[storage_mapper("projects")]
    fn projects(&self) -> Mapping<u64, Project<Self::Api>>;
}
