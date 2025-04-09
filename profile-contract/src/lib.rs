#![no_std]

multiversx_sc::imports!();

pub mod storage;
pub mod models;
pub mod user;
pub mod project;
pub mod star;

use user::*;
use star::*;

#[multiversx_sc::contract]
pub trait ProfileContract:
    storage::StorageModule +
    user::UserModule +
    project::ProjectModule {}
