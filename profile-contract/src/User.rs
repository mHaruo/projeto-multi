// user.rs
use multiversx_sc::api::ManagedTypeApi;
use multiversx_sc::contract_base::ContractBase;
use multiversx_sc::imports::*;
use multiversx_sc::storage::StorageMapper;
use multiversx_sc::types::ManagedBuffer;

use crate::user_model::User;

#[multiversx_sc::module]
pub trait UserModule: ContractBase {
    #[endpoint]
    fn register_user(
        &self,
        name: ManagedBuffer,
        linkedin: ManagedBuffer,
        github: ManagedBuffer,
        twitter: ManagedBuffer,
    ) {
        let caller = self.blockchain().get_caller();
        let user = User {
            name,
            linkedin,
            github,
            twitter,
            stars: 0,
        };

        self.users().insert(caller, user);
    }

    #[endpoint]
    fn give_star(&self, to: ManagedAddress) {
        let from = self.blockchain().get_caller();
        require!(from != to, "Você não pode dar estrela a si mesmo");

        let timestamp = self.blockchain().get_block_timestamp();
        let current_day = timestamp / 86400;

        // Pega o dia da última estrela dada por esse usuário
        let last_day = self.last_star_day(&from).get();
        let mut count = self.daily_star_count(&from).get();

        // Se mudou o dia, reseta o contador
        if current_day != last_day {
            count = 0;
            self.daily_star_count(&from).set(0);
            self.last_star_day(&from).set(current_day);
        }

        require!(count < 10, "Você atingiu o limite de 10 estrelas hoje");

        let base_cost = BigUint::from(10_000_000_000_000u64);
        let cost = &base_cost << count;

        let payment = self.call_value().egld_value();
        require!(payment >= cost, "Valor insuficiente para dar estrela");

        self.users().update(&to, |user| {
            user.stars += 1;
        });

        self.daily_star_count(&from).set(count + 1);
        self.last_star_day(&from).set(current_day);
    }

    #[view]
    fn get_user(&self, address: ManagedAddress) -> OptionalValue<User<Self::Api>> {
        self.users().get(address)
    }

    #[storage_mapper("users")]
    fn users(&self) -> MapMapper<ManagedAddress, User<Self::Api>>;

    #[storage_mapper("daily_star_count")]
    fn daily_star_count(&self, addr: &ManagedAddress) -> SingleValueMapper<u32>;

    #[storage_mapper("daily_star_count")]
    fn daily_star_count(&self, addr: &ManagedAddress) -> SingleValueMapper<u32>;

    #[storage_mapper("last_star_day")]
    fn last_star_day(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;
}
