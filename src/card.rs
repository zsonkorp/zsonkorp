pub(crate) const RANK_COUNT: u8 = 13;
pub(crate) const SUIT_COUNT: u8 = 4;

pub(crate) struct Card {
    val: u8
}

#[derive(PartialEq, Debug)]
pub enum Suit {
    HEARTS,
    SPADES,
    CLUBS,
    DIAMONDS
}

impl From<u8> for Suit {
    fn from(value: u8) -> Self {
        match value {
            0 => Suit::HEARTS,
            1 => Suit::SPADES,
            2 => Suit::CLUBS,
            3 => Suit::DIAMONDS,
            _ => panic!("Invalid suit")
        }
    }
}

impl From<Suit> for u8 {
    fn from(value: Suit) -> Self {
        match value {
            Suit::HEARTS => 0,
            Suit::SPADES => 1,
            Suit::CLUBS => 2,
            Suit::DIAMONDS => 3
        }
    }
}

impl Card {
    pub fn new(rank: u8, suit: Suit) -> Self {

        if rank == 0 || rank > RANK_COUNT {
            panic!()
        }

        Card {
            val: u8::from(suit) * RANK_COUNT + (rank - 1)
        }
    }

    pub fn get_rank(&self) -> u8 {
        self.val % RANK_COUNT + 1
    }

    pub fn get_suit(&self) -> Suit {
        Suit::from(self.val / RANK_COUNT)
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
        assert_eq!(u8::from(Suit::HEARTS), 0);
        assert_eq!(u8::from(Suit::SPADES), 1);
        assert_eq!(u8::from(Suit::CLUBS), 2);
        assert_eq!(u8::from(Suit::DIAMONDS), 3);
    }

    #[test]
    fn ord_to_suit() {
        assert_eq!(Suit::from(0), Suit::HEARTS);
        assert_eq!(Suit::from(1), Suit::SPADES);
        assert_eq!(Suit::from(2), Suit::CLUBS);
        assert_eq!(Suit::from(3), Suit::DIAMONDS);
    }

    #[test]
    #[should_panic]
    fn invalid_ord_to_suit_edge() {
        let _ = Suit::from(4);
    }

    #[test]
    fn create_card_suit() {
        let card = Card::new(1, Suit::HEARTS);
        assert_eq!(card.val, 0);

        let card = Card::new(1, Suit::SPADES);
        assert_eq!(card.val, RANK_COUNT);

        let card = Card::new(1, Suit::CLUBS);
        assert_eq!(card.val, RANK_COUNT * 2);

        let card = Card::new(1, Suit::DIAMONDS);
        assert_eq!(card.val, RANK_COUNT * 3);
    }

    #[test]
    fn create_card_rank() {
        let card = Card::new(1, Suit::HEARTS);
        assert_eq!(card.val, 0);

        let card = Card::new(2, Suit::HEARTS);
        assert_eq!(card.val, 1);

        let card = Card::new(6, Suit::HEARTS);
        assert_eq!(card.val, 5);

        let card = Card::new(7, Suit::HEARTS);
        assert_eq!(card.val, 6);

        let card = Card::new(13, Suit::HEARTS);
        assert_eq!(card.val, 12);

        let card = Card::new(12, Suit::HEARTS);
        assert_eq!(card.val, 11);
    }

    // #[test]
    fn create_card_rank_suit_mix() {
        todo!()
    }

    #[test]
    #[should_panic]
    fn create_card_invalid_rank() {
        todo!()
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
        assert_eq!(card.get_suit(), Suit::HEARTS);

        let card = Card { val: 13 };
        assert_eq!(card.get_suit(), Suit::SPADES);

        let card = Card { val: 26 };
        assert_eq!(card.get_suit(), Suit::CLUBS);

        let card = Card { val: 39 };
        assert_eq!(card.get_suit(), Suit::DIAMONDS);

        let card = Card { val: 5 };
        assert_eq!(card.get_suit(), Suit::HEARTS);

        let card = Card { val: 12 };
        assert_eq!(card.get_suit(), Suit::HEARTS);
    }
}