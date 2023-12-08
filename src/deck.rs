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
            0 => panic!(),   //very nice
            _ => {
                let mut cards = Vec::with_capacity(
                    (count * RANK_COUNT as u32 * SUIT_COUNT as u32) as usize
                );

                for _ in 0..count {
                    for suit_ord in 0..SUIT_COUNT {
                        for rank in 1..(RANK_COUNT + 1) {
                            cards.push(Card::new(rank, suit_ord.into()));
                        }
                    }
                }

                return Deck { cards };
            }
        }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }
}

#[cfg(test)]
mod tests {
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

        assert_eq!(deck.cards.len(), 52 * deck_count);

        for i in 0..deck_count {
            for j in 0..52 {
                assert_eq!(deck.cards[i * 52 + j].get_val() as usize, j);
            }
        }
    }

    #[test]
    #[should_panic]
    fn create_invalid_multi_deck() {
        todo!()
    }
}