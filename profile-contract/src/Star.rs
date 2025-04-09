pub mod star {
    use super::*;

    #[storage_mapper("last_star_day")]
    pub fn last_star_day(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("daily_star_count")]
    pub fn daily_star_count(&self, addr: &ManagedAddress) -> SingleValueMapper<u32>;

    #[endpoint(giveStar)]
    pub fn give_star(&self, to: ManagedAddress) {
        let from = self.blockchain().get_caller();
        let timestamp = self.blockchain().get_block_timestamp();
        let current_day = timestamp / 86400;

        let last_day = self.last_star_day(&from).get();
        let mut count = self.daily_star_count(&from).get();

        if current_day != last_day {
            count = 0;
        }

        let base_cost = BigUint::from(10_000_000_000_000u64); // 0.01 EGLD
        let cost = &base_cost << count;

        self.send().direct_egld(&self.blockchain().get_owner_address(), &cost);
        self.last_star_day(&from).set(current_day);
        self.daily_star_count(&from).set(count + 1);

        self.users().update(&to, |user| user.stars += 1);
    }

    #[view(getStarCost)]
    pub fn get_star_cost(&self, from: ManagedAddress) -> BigUint {
        let timestamp = self.blockchain().get_block_timestamp();
        let current_day = timestamp / 86400;

        let last_day = self.last_star_day(&from).get();
        let mut count = self.daily_star_count(&from).get();

        if current_day != last_day {
            count = 0;
        }

        let base_cost = BigUint::from(10_000_000_000_000u64);
        &base_cost << count
    }
}
