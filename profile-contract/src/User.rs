use crate::ManagedTypeApi;
use multiversx_sc::proxy_imports::{TopEncode, NestedEncode, TopDecode, NestedDecode};
use multiversx_sc::types::ManagedBuffer;
use multiversx_sc::codec;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct UserImpl<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub stars: u32,
    pub twitter: ManagedBuffer<M>,
    pub telegram: ManagedBuffer<M>,
    pub instagram: ManagedBuffer<M>,
}
