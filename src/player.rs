use std::hash::{Hash, Hasher};

pub struct Player {
    pub id: String
}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq<Self> for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(other)
    }
}