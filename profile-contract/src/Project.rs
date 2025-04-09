use crate::ManagedTypeApi;
use multiversx_sc::proxy_imports::{TopEncode, NestedEncode, TopDecode, NestedDecode};
use multiversx_sc::types::{ManagedBuffer, ManagedType};

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct Projec<M: ManagedTypeApi> {
    pub id: u32,
    pub title: ManagedBuffer<M>,
    pub yes_votes: u32,
    pub no_votes: u32,
}
