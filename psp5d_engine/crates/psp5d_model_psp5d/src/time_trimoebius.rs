use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum K1Policy {
    MirrorOnCommit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriMoebiusState {
    pub k1: u64,
    pub k2: u64,
    pub nc_tag: String,
    pub policy: K1Policy,
}

impl TriMoebiusState {
    pub fn tick(&mut self) {
        self.k2 = self.k2.saturating_add(1);
    }

    pub fn commit(&mut self) {
        match self.policy {
            K1Policy::MirrorOnCommit => {
                self.k1 = self.k2;
            }
        }
    }
}
