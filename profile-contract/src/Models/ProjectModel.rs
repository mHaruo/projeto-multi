use multiversx_sc::types::{ManagedBuffer, ManagedVec, ManagedAddress, BigUint};
use multiversx_sc::api::BlockchainApi;
use multiversx_sc::types::heap::Vec as ScVec;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Project<M: BlockchainApi> {
    pub id: u64,
    pub title: ManagedBuffer<M>,
    pub votes_in_favor: u32,
    pub votes_against: u32,
}