use crate::card::{Card, RANK_COUNT, SUIT_COUNT};
use rand::thread_rng;
use rand::seq::SliceRandom;

struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        Deck::new_multi(1)
    }

    pub fn new_multi(count: u32) -> Self {
        match count {
            0 => Deck{ cards: Vec::new() },
            _ => {
                let mut cards = Vec::with_capacity(
                    (count * RANK_COUNT as u32 * SUIT_COUNT as u32) as usize
                );

                for _ in 0..count {
                    for suit_ord in 0..SUIT_COUNT {
                        for rank in 1..(RANK_COUNT + 1) {
                            cards.push(Card::new(rank, suit_ord.try_into().unwrap()).unwrap());
                        }
                    }
                }

                Deck { cards }
            }
        }
    }

    //TODO: implement split interval: startIdx & endIdx
    pub fn split(&mut self, index: usize) -> Self {
        if self.cards.len() <= 1 {
            //TODO: error here
            panic!()
        }

        if index >= self.cards.len() - 1 {
            //TODO: error here
            panic!()
        }

        Deck {
            cards: self.cards.split_off(index+ 1)
        }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}

#[cfg(test)]
mod tests {
    use crate::card::Suit::Hearts;
    use super::*;

    #[test]
    fn create_single_deck() {
        let deck = Deck::new();

        assert_eq!(deck.cards.len(), 52);

        for i in 0..52 {
            assert_eq!(deck.cards[i].get_val() as usize, i);
        }
    }

    #[test]
    fn create_multi_deck() {
        let deck_count = 6;
        let deck = Deck::new_multi(deck_count);

        assert_eq!(deck.cards.len(), (52 * deck_count) as usize);

        for i in 0..deck_count {
            for j in 0..52 {
                assert_eq!(deck.cards[(i * 52 + j) as usize].get_val(), j as u8);
            }
        }
    }

    #[test]
    fn create_empty_deck() {
        let deck = Deck::new_multi(0);
        assert!(deck.cards.is_empty());
    }

    #[test]
    fn split_all_but_one() {
        let mut orig_deck = Deck {
            cards: vec![
                Card::new(1, Hearts).unwrap(),
                Card::new(2, Hearts).unwrap(),
                Card::new(3, Hearts).unwrap(),
                Card::new(4, Hearts).unwrap(),
            ]
        };

        let new_deck = orig_deck.split(0);

        assert_eq!(orig_deck.cards.len(), 1);
        assert_eq!(orig_deck.cards[0], Card::new(1, Hearts).unwrap());

        assert_eq!(new_deck.cards.len(), 3);
        for (i, card) in new_deck.cards.iter().enumerate() {
            assert_eq!(card.get_rank() as usize, i + 2);
            assert_eq!(card.get_suit(), Hearts);
        }
    }

    #[test]
    fn split_one_end() {
        let mut orig_deck = Deck {
            cards: vec![
                Card::new(1, Hearts).unwrap(),
                Card::new(2, Hearts).unwrap(),
                Card::new(3, Hearts).unwrap(),
                Card::new(4, Hearts).unwrap(),
            ]
        };

        let new_deck = orig_deck.split(2);

        assert_eq!(orig_deck.cards.len(), 3);

        for (i, card) in orig_deck.cards.iter().enumerate() {
            assert_eq!(card.get_rank() as usize, i + 1);
            assert_eq!(card.get_suit(), Hearts);
        }

        assert_eq!(new_deck.cards.len(), 1);
        assert_eq!(new_deck.cards[0], Card::new(4, Hearts).unwrap());

    }

    #[test]
    fn split_middle() {
        let mut orig_deck = Deck {
            cards: vec![
                Card::new(1, Hearts).unwrap(),
                Card::new(2, Hearts).unwrap(),
                Card::new(3, Hearts).unwrap(),
                Card::new(4, Hearts).unwrap(),
            ]
        };

        let new_deck = orig_deck.split(1);

        assert_eq!(orig_deck.cards.len(), 2);
        assert_eq!(orig_deck.cards[0], Card::new(1, Hearts).unwrap());
        assert_eq!(orig_deck.cards[1], Card::new(2, Hearts).unwrap());

        assert_eq!(new_deck.cards.len(), 2);
        assert_eq!(new_deck.cards[0], Card::new(3, Hearts).unwrap());
        assert_eq!(new_deck.cards[1], Card::new(4, Hearts).unwrap());
    }

    #[test]
    #[should_panic]
    fn invalid_split() {
        todo!()
    }
}