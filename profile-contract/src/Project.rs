use crate::storage::StorageModule;
use multiversx_sc::types::{ManagedBuffer};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ProjectModule: StorageModule {
    #[endpoint(createProject)]
    fn create_project(&self, title: ManagedBuffer) {
        let id = self.next_project_id().get();
        self.projects(&id).set(title);
        self.project_votes_yes(&id).set(0);
        self.project_votes_no(&id).set(0);
        self.next_project_id().set(id + 1);
    }

    #[endpoint(voteProject)]
    fn vote_project(&self, id: u64, vote: bool) {
        if vote {
            let current = self.project_votes_yes(&id).get();
            self.project_votes_yes(&id).set(current + 1);
        } else {
            let current = self.project_votes_no(&id).get();
            self.project_votes_no(&id).set(current + 1);
        }
    }

    #[view(getProject)]
    fn get_project(&self, id: u64) -> (ManagedBuffer<Self::Api>, u32, u32) {
        let title = self.projects(&id).get();
        let yes = self.project_votes_yes(&id).get();
        let no = self.project_votes_no(&id).get();
        (title, yes, no)
    }
}
