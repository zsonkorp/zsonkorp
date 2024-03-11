use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::deck::Deck;

struct DeckSerializerContainer<'a> {
    deck: &'a Deck,
}
impl<'a> DeckSerializerContainer<'a> {
    pub fn new(deck: &'a Deck) -> Self {
        DeckSerializerContainer {deck}
    }
}

impl<'a> Serialize for DeckSerializerContainer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut serializer = serializer.serialize_struct("Deck", 2)?;
        serializer.serialize_field("dealt_cards", &self.deck.get_dealt_cards())?;
        serializer.serialize_field("remain_size", &self.deck.remain_len())?;
        serializer.end()
    }
}

struct CtaDeckSerializerContainer<'a> {
    deck: &'a Deck,
    show_top: bool,
    show_bottom: bool
}
impl<'a> CtaDeckSerializerContainer<'a> {
    pub fn new(deck: &'a Deck, show_top: bool, show_bottom: bool) -> Self {
       CtaDeckSerializerContainer {deck, show_top, show_bottom}
    }
}

impl<'a> Serialize for CtaDeckSerializerContainer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut serializer = serializer.serialize_struct("Deck", 3)?;
        serializer.serialize_field("size", &self.deck.remain_len())?;
        if self.show_top {
            serializer.serialize_field("top", self.deck.get_top())?;
        }

        if self.show_bottom {
            serializer.serialize_field("bottom", self.deck.get_bottom())?;
        }

        serializer.end()
    }
}