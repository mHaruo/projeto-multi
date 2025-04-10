#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait ProfileContract: User::UserImpl + Project::ProjectImpl + Storage::StorageModule<Self::Api> {
    #[init]
    fn init(&self) {

    }
}

pub mod Project;
pub mod User;
pub mod Storage;

