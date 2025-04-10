use crate::ManagedTypeApi;
use multiversx_sc::codec;
use multiversx_sc_codec::NestedDecodeInput;
use multiversx_sc::proxy_imports::{TopEncode, NestedEncode, TopDecode, NestedDecode};
use multiversx_sc::types::{ManagedBuffer, ManagedType};
use crate::SingleValueMapper;
use multiversx_sc::module;
use multiversx_sc::derive::TypeAbi;
use multiversx_sc::imports::StorageMapper;



#[derive(TopEncode, TopDecode, TypeAbi, NestedEncode, NestedDecode)]
pub struct ProjectModel<M: ManagedTypeApi> {
    pub title: ManagedBuffer<M>,
    pub vote_yes: u32,
    pub vote_no: u32,
}

#[module]
pub trait ProjectImpl {
    #[storage_mapper("project")]
    fn project(&self, project_id: u32) -> SingleValueMapper<ProjectModel<Self::Api>>;

    #[endpoint(createProject)]
    fn create_project(
        &self,
        project_id: u32,
        title: ManagedBuffer,
    ) {
        let project = ProjectModel {
            title,
            vote_yes: 0,
            vote_no: 0,
        };
        self.project(project_id).set(project);
    }
}