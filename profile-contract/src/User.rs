use crate::ManagedTypeApi;
use multiversx_sc::proxy_imports::{TopEncode, NestedEncode, TopDecode, NestedDecode};
use multiversx_sc::types::ManagedBuffer;
use multiversx_sc::codec;
use multiversx_sc::module;
use crate::SingleValueMapper;
use multiversx_sc::derive::TypeAbi;



#[derive(TopEncode, TopDecode, TypeAbi, NestedEncode, NestedDecode)]
pub struct UserModel<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub stars: u32,
    pub twitter: ManagedBuffer<M>,
    pub telegram: ManagedBuffer<M>,
    pub instagram: ManagedBuffer<M>,
}

#[module]
pub trait UserImpl {
    #[storage_mapper("user")]
    fn user(&self, user_id: u32) -> SingleValueMapper<UserModel<Self::Api>>;

    #[endpoint(createUser)]
    fn create_user(
        &self,
        user_id: u32,
        name: ManagedBuffer,
        twitter: ManagedBuffer,
        telegram: ManagedBuffer,
        instagram: ManagedBuffer,
    ) {
        let user = UserModel {
            name,
            stars: 0,
            twitter,
            telegram,
            instagram,
        };
        self.user(user_id).set(user);
    }
}