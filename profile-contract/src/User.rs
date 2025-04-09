pub mod user {
    use super::*;

    #[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq)]
    pub struct UserModel<M: ManagedTypeApi> {
        pub name: ManagedBuffer<M>,
        pub linkedin: ManagedBuffer<M>,
        pub github: ManagedBuffer<M>,
        pub twitter: ManagedBuffer<M>,
        pub stars: u32,
    }

    #[storage_mapper("users")]
    pub fn users(&self) -> MapMapper<ManagedAddress, UserModel<Self::Api>>;

    #[view(getUser)]
    pub fn get_user(&self, addr: ManagedAddress) -> UserModel<Self::Api> {
        self.users().get(&addr)
    }

    #[endpoint(addUser)]
    pub fn add_user(
        &self,
        name: ManagedBuffer,
        linkedin: ManagedBuffer,
        github: ManagedBuffer,
        twitter: ManagedBuffer,
    ) {
        let caller = self.blockchain().get_caller();
        let user = UserModel {
            name,
            linkedin,
            github,
            twitter,
            stars: 0,
        };
        self.users().insert(caller, user);
    }

    #[view(getUserRank)]
    pub fn get_user_rank(&self, addr: ManagedAddress) -> u8 {
        let stars = self.users().get(&addr).stars;
        if stars >= 5 {
            3
        } else if stars >= 2 {
            2
        } else {
            1
        }
    }

    #[view(getVotingPower)]
    pub fn get_voting_power(&self, addr: ManagedAddress) -> u8 {
        let stars = self.users().get(&addr).stars;
        if stars >= 5 {
            3
        } else if stars >= 2 {
            2
        } else {
            1
        }
    }
}
