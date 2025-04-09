#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait ProfileContract {
    #[init]
    fn init(&self) {}

    #[view(getVersion)]
    fn get_version(&self) -> u32 {
        1
    }
}
