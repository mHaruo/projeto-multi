// user_model.rs
use multiversx_sc::types::ManagedBuffer;
use multiversx_sc::types::ManagedAddress;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct User<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub linkedin: ManagedBuffer<M>,
    pub github: ManagedBuffer<M>,
    pub twitter: ManagedBuffer<M>,
    pub stars: u32,
}
