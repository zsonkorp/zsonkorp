use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{Error, Visitor};

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
        self.id.eq(&other.id)
    }
}

impl Eq for Player {

}

struct PlayerVisitor {

}
impl PlayerVisitor {
    pub fn new() -> Self {
        PlayerVisitor {}
    }
}

impl Visitor<'_> for PlayerVisitor {
    type Value = Player;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "a string for player id")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        Ok(Player {
            id: v.to_string()
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
        Ok(Player {
            id: v
        })
    }
}
impl<'de> Deserialize<'de> for Player {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(PlayerVisitor::new())
    }
}