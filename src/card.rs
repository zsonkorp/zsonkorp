use std::fmt;
use std::fmt::{Debug, Formatter};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use thiserror::Error;
use crate::card;

const RANK_COUNT: u8 = 13;
const SUIT_COUNT: u8 = 4;

pub fn get_rank_count() -> u8 {
    RANK_COUNT
}

pub fn get_suit_count() -> u8 {
    SUIT_COUNT
}

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Invalid suit ordinal value: {0}")]
    SuitOrdinal(u8),
    #[error("Invalid rank value: {0}")]
    InvalidRank(u8)
}

#[derive(PartialEq)]
pub(crate) struct Card {
    val: u8
}

impl Card {
    pub fn new(rank: u8, suit: Suit) -> Result<Card, card::Error> {

        if rank == 0 || rank > RANK_COUNT {
            return Err(Error::InvalidRank(rank));
        }

        Ok(Card {
            val: u8::from(suit) * RANK_COUNT + (rank - 1)
        })
    }

    pub fn get_rank(&self) -> u8 {
        self.val % RANK_COUNT + 1
    }

    pub fn get_suit(&self) -> Suit {
        Suit::try_from(self.val / RANK_COUNT).unwrap()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Card {{ val: {}, rank: {}, suit: {} }}", self.val, self.get_rank(), self.get_suit())
    }
}
impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(PartialEq, Debug, Serialize)]
pub enum Suit {
    Hearts,
    Spades,
    Clubs,
    Diamonds
}


impl fmt::Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}



impl TryFrom<u8> for Suit {
    type Error = card::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Suit::Hearts),
            1 => Ok(Suit::Spades),
            2 => Ok(Suit::Clubs),
            3 => Ok(Suit::Diamonds),
            _ => Err(Error::SuitOrdinal(value))
        }
    }
}

impl From<Suit> for u8 {
    fn from(value: Suit) -> Self {
        match value {
            Suit::Hearts => 0,
            Suit::Spades => 1,
            Suit::Clubs => 2,
            Suit::Diamonds => 3
        }
    }
}

impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut serializer = serializer.serialize_struct("Card", 3)?;
        serializer.serialize_field("val", &self.val)?;
        serializer.serialize_field("rank", &self.get_rank())?;
        serializer.serialize_field("suit", &self.get_suit())?;
        serializer.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Card {
        pub fn get_val(&self) -> u8 {
            self.val
        }
    }

    #[test]
    fn suit_to_ord() {
        assert_eq!(u8::from(Suit::Hearts), 0);
        assert_eq!(u8::from(Suit::Spades), 1);
        assert_eq!(u8::from(Suit::Clubs), 2);
        assert_eq!(u8::from(Suit::Diamonds), 3);
    }

    #[test]
    fn ord_to_suit() -> Result<(), card::Error> {
        assert_eq!(Suit::try_from(0)?, Suit::Hearts);
        assert_eq!(Suit::try_from(1)?, Suit::Spades);
        assert_eq!(Suit::try_from(2)?, Suit::Clubs);
        assert_eq!(Suit::try_from(3)?, Suit::Diamonds);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn invalid_ord_to_suit_edge() {
        Suit::try_from(4).unwrap();
    }

    #[test]
    fn create_card_suit() -> Result<(), card::Error>{
        let card = Card::new(1, Suit::Hearts)?;
        assert_eq!(card.val, 0);

        let card = Card::new(1, Suit::Spades)?;
        assert_eq!(card.val, RANK_COUNT);

        let card = Card::new(1, Suit::Clubs)?;
        assert_eq!(card.val, RANK_COUNT * 2);

        let card = Card::new(1, Suit::Diamonds)?;
        assert_eq!(card.val, RANK_COUNT * 3);
        Ok(())
    }

    #[test]
    fn create_card_rank() -> Result<(), card::Error>{
        let card = Card::new(1, Suit::Hearts)?;
        assert_eq!(card.val, 0);

        let card = Card::new(2, Suit::Hearts)?;
        assert_eq!(card.val, 1);

        let card = Card::new(6, Suit::Hearts)?;
        assert_eq!(card.val, 5);

        let card = Card::new(7, Suit::Hearts)?;
        assert_eq!(card.val, 6);

        let card = Card::new(13, Suit::Hearts)?;
        assert_eq!(card.val, 12);

        let card = Card::new(12, Suit::Hearts)?;
        assert_eq!(card.val, 11);
        Ok(())
    }

    // #[test]
    fn create_card_rank_suit_mix() {
        todo!()
    }

    #[test]
    #[should_panic]
    fn create_card_invalid_rank_0() {
        Card::new(0, Suit::Hearts).unwrap();
    }

    #[test]
    fn create_card_invalid_rank_above_count() {
        assert_eq!(
            Card::new(RANK_COUNT + 1, Suit::Hearts).unwrap_err(),
            Error::InvalidRank(RANK_COUNT + 1)
        );

        assert_eq!(
            Card::new(RANK_COUNT + 2, Suit::Hearts).unwrap_err(),
            Error::InvalidRank(RANK_COUNT + 2)
        );

        assert_eq!(
            Card::new(RANK_COUNT + 100, Suit::Hearts).unwrap_err(),
            Error::InvalidRank(RANK_COUNT + 100)
        );
    }

    #[test]
    fn get_rank() {
        let card = Card { val: 0 };
        assert_eq!(card.get_rank(), 1);

        let card = Card { val: 13 };
        assert_eq!(card.get_rank(), 1);

        let card = Card { val: 5 };
        assert_eq!(card.get_rank(), 6);

        let card = Card { val: 12 };
        assert_eq!(card.get_rank(), 13);

        let card = Card { val: 13 };
        assert_eq!(card.get_rank(), 1);

        let card = Card { val: 13 };
        assert_eq!(card.get_rank(), 1);
    }

    #[test]
    fn get_suit() {
        let card = Card { val: 0 };
        assert_eq!(card.get_suit(), Suit::Hearts);

        let card = Card { val: 13 };
        assert_eq!(card.get_suit(), Suit::Spades);

        let card = Card { val: 26 };
        assert_eq!(card.get_suit(), Suit::Clubs);

        let card = Card { val: 39 };
        assert_eq!(card.get_suit(), Suit::Diamonds);

        let card = Card { val: 5 };
        assert_eq!(card.get_suit(), Suit::Hearts);

        let card = Card { val: 12 };
        assert_eq!(card.get_suit(), Suit::Hearts);
    }
}