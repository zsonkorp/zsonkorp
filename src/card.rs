const RANK_COUNT: u8 = 13;
const SUIT_COUNT: u8 = 4;

pub(crate) struct Card {
    val: u8
}

enum Suit {
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

// impl Suit {
//     fn to_ordinal(&self) -> u8 {
//         match self {
//             Suit::HEARTS => 0,
//             Suit::SPADES => 1,
//             Suit::CLUBS => 2,
//             Suit::DIAMONDS => 3
//         }
//     }
//
//     fn from_ordinal(ordinal: u8) -> Suit {
//         match ordinal {
//             0 => Suit::HEARTS,
//             1 => Suit::SPADES,
//             2 => Suit::CLUBS,
//             3 => Suit::DIAMONDS,
//             _ => panic!("Invalid suit")
//         }
//     }
// }

impl Card {
    pub fn new(rank: u8, suit: Suit) -> Self {
        Card {
            val: suit.into() * RANK_COUNT + rank
        }
    }

    pub fn get_rank(&self) -> u8 {
        self.val % SUIT_COUNT
    }

    pub fn get_suit(&self) -> Suit {
        Suit::from(self.val / SUIT_COUNT)
    }
}