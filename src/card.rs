pub(crate) const RANK_COUNT: u8 = 13;
pub(crate) const SUIT_COUNT: u8 = 4;

#[derive(PartialEq, Debug)]
pub(crate) struct Card {
    val: u8
}

#[derive(PartialEq, Debug)]
pub enum Suit {
    Hearts,
    Spades,
    Clubs,
    Diamonds
}

impl From<u8> for Suit {
    fn from(value: u8) -> Self {
        match value {
            0 => Suit::Hearts,
            1 => Suit::Spades,
            2 => Suit::Clubs,
            3 => Suit::Diamonds,
            _ => panic!("Invalid suit")
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
        assert_eq!(u8::from(Suit::Hearts), 0);
        assert_eq!(u8::from(Suit::Spades), 1);
        assert_eq!(u8::from(Suit::Clubs), 2);
        assert_eq!(u8::from(Suit::Diamonds), 3);
    }

    #[test]
    fn ord_to_suit() {
        assert_eq!(Suit::from(0), Suit::Hearts);
        assert_eq!(Suit::from(1), Suit::Spades);
        assert_eq!(Suit::from(2), Suit::Clubs);
        assert_eq!(Suit::from(3), Suit::Diamonds);
    }

    #[test]
    #[should_panic]
    fn invalid_ord_to_suit_edge() {
        let _ = Suit::from(4);
    }

    #[test]
    fn create_card_suit() {
        let card = Card::new(1, Suit::Hearts);
        assert_eq!(card.val, 0);

        let card = Card::new(1, Suit::Spades);
        assert_eq!(card.val, RANK_COUNT);

        let card = Card::new(1, Suit::Clubs);
        assert_eq!(card.val, RANK_COUNT * 2);

        let card = Card::new(1, Suit::Diamonds);
        assert_eq!(card.val, RANK_COUNT * 3);
    }

    #[test]
    fn create_card_rank() {
        let card = Card::new(1, Suit::Hearts);
        assert_eq!(card.val, 0);

        let card = Card::new(2, Suit::Hearts);
        assert_eq!(card.val, 1);

        let card = Card::new(6, Suit::Hearts);
        assert_eq!(card.val, 5);

        let card = Card::new(7, Suit::Hearts);
        assert_eq!(card.val, 6);

        let card = Card::new(13, Suit::Hearts);
        assert_eq!(card.val, 12);

        let card = Card::new(12, Suit::Hearts);
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