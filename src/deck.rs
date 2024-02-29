use crate::card::{Card, get_suit_count, get_rank_count};
use rand::thread_rng;
use rand::seq::SliceRandom;

pub(crate) struct Deck {
    cards: Vec<Card>,
    next_idx: usize
}

impl Default for Deck {
    fn default() -> Self {
        Deck::new_multi(1)
    }
}

impl Deck {
    pub fn new() -> Self {
        Deck::new_multi(0)
    }

    pub fn new_multi(count: u32) -> Self {
        match count {
            0 => Deck{
                cards: Vec::new(),
                next_idx: 0
            },

            _ => {
                let mut cards = Vec::with_capacity(
                    (count * get_rank_count() as u32 * get_suit_count() as u32) as usize
                );

                for _ in 0..count {
                    for suit_ord in 0..get_suit_count() {
                        for rank in 1..(get_rank_count() + 1) {
                            cards.push(Card::new(rank, suit_ord.try_into().unwrap()).unwrap());
                        }
                    }
                }

                Deck { cards, next_idx: 0 }
            }
        }
    }

    // Returns the newly split off deck: (index, len-1)
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
            cards: self.cards.split_off(index+ 1),
            next_idx: 0
        }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());

        //shuffling serves to reset the deck
        self.next_idx = 0;
    }

    pub fn deal(&mut self) -> Option<&Card> {
        if self.next_idx == self.cards.len() {
            return None
        }

        let card = &self.cards[self.next_idx];
        self.next_idx += 1;

        Some(card)
    }

    pub fn deal_multi(&mut self, count: usize) -> Option<&[Card]> {
        if self.next_idx == self.cards.len() || self.next_idx + count >= self.cards.len() {
            return None
        }

        let ret = Some(&self.cards[self.next_idx..self.next_idx + count]);
        self.next_idx += count;
        ret
    }

    pub fn get_dealt_cards(&self) -> &[Card] {
        &self.cards[0..self.next_idx]
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::card::Suit::Hearts;
    use super::*;

    #[test]
    fn create_empty_deck() {
        let deck = Deck::new();
        assert!(deck.cards.is_empty())
    }

    #[test]
    fn create_default_deck() {
        let deck = Deck::default();

        assert_eq!(deck.cards.len(), 52);

        for i in 0..52 {
            assert_eq!(deck.cards[i].get_val() as usize, i);
        }
    }

    #[test]
    fn create_single_deck() {
        let deck = Deck::new_multi(1);

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
    fn split_all_but_one() {
        let mut orig_deck = Deck {
            cards: vec![
                Card::new(1, Hearts).unwrap(),
                Card::new(2, Hearts).unwrap(),
                Card::new(3, Hearts).unwrap(),
                Card::new(4, Hearts).unwrap(),
            ],
            next_idx: 0
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
            ],
            next_idx: 0
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
            ],
            next_idx: 0
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