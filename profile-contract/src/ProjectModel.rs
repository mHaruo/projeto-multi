use crate::Project::*;
use multiversx_sc::types::ManagedBuffer;
use multiversx_sc::api::ManagedTypeApi;

impl<M: ManagedTypeApi> ProjectImpl<M> {
    pub fn new(id: u32, title: ManagedBuffer<M>) -> Self {
        Self {
            id,
            title,
            votes_yes: 0,
            votes_no: 0,
        }
    }
}
