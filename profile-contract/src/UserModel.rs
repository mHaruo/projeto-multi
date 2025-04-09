use crate::User::*;
use multiversx_sc::types::ManagedBuffer;
use multiversx_sc::api::ManagedTypeApi;

impl<M: ManagedTypeApi> UserImpl<M> {
    pub fn new(
        name: ManagedBuffer<M>,
        twitter: ManagedBuffer<M>,
        telegram: ManagedBuffer<M>,
        instagram: ManagedBuffer<M>,
    ) -> Self {
        Self {
            name,
            stars: 0,
            twitter,
            telegram,
            instagram,
        }
    }
}
