/* use crate::User::UserImpl;
use crate::ManagedTypeApi;

use multiversx_sc::storage::{SingleValueMapper, MapMapper};
use multiversx_sc::types::{ManagedAddress, ManagedBuffer, BigUint, ManagedVec};
use multiversx_sc::proxy_imports::ManagedTypeApi;

pub trait StarModule: ContractBase {
    // ⭐ Mapeamento diário de estrelas por (usuário -> dia)
    fn daily_star_count(&self) -> MapMapper<Self::Api, (ManagedAddress<Self::Api>, u64), u32> {
        MapMapper::new(b"daily_star_count")
    }

    // ⭐ Total de estrelas que um usuário já recebeu
    fn user_star_total(&self) -> MapMapper<Self::Api, ManagedAddress<Self::Api>, u32> {
        MapMapper::new(b"user_star_total")
    }

    // ⭐ Limite de estrelas que cada usuário pode dar por dia
    fn daily_star_limit(&self) -> SingleValueMapper<Self::Api, u32> {
        SingleValueMapper::new(b"daily_star_limit")
    }

    // ⭐ Função para calcular badge de reputação
    fn get_user_badge(&self, stars: u32) -> u8 {
        match stars {
            0 | 1 => 1, // Bronze
            2 => 2,     // Prata
            _ => 3,     // Ouro
        }
    }

    // ⭐ Peso do voto baseado no badge
    fn get_vote_weight(&self, stars: u32) -> u32 {
        match self.get_user_badge(stars) {
            1 => 1, // Bronze
            2 => 2, // Prata
            3 => 3, // Ouro
            _ => 1,
        }
    }
}
 */