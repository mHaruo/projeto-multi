#![no_std]

multiversx_sc::imports!();

pub mod storage;
pub mod models;
pub mod user;
pub mod project;

#[multiversx_sc::contract]
pub trait ProfileContract:
    storage::StorageModule +
    user::UserModule +
    project::ProjectModule {}
