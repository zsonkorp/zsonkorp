use std::ops::Deref;
use crate::deck::Deck;
use crate::config::{CtaWagerType, FtsWagerType};
use crate::config;
use crate::game::{Game, GameType};
use anyhow::Result;
use serde::Serialize;
use crate::game::Error::InvalidTransition;
use crate::payout::Payout;
use crate::state::State;
use crate::state::GameState::*;
use crate::state::State::*;
use crate::transition::GameTransition::{End, Start};
use crate::state::CtaState::*;
use crate::transition::cta::Error::{InvalidCut, UnoptimalCut};
use crate::transition::CtaTransition::*;
use crate::transition::Transition;

struct DeckInfo {
    deck: Deck,
    top_reveal: bool,
    bottom_reveal: bool
}

impl DeckInfo {

    pub fn new(deck: Deck) -> Self {
        DeckInfo {deck, top_reveal: false, bottom_reveal: false}
    }
    pub fn new_with_info(deck: Deck, top_reveal: bool, bottom_reveal: bool) -> Self {
        DeckInfo { deck, top_reveal, bottom_reveal }
    }
}

pub struct Cta {
    config: config::Cta,
    state: State,
    deck_pool: Vec<DeckInfo>,
    bottom_deck_index: usize,
    enforce_optimal_cut: bool,
    cut_pair: bool
}

impl Cta {
    pub fn new(config: config::Cta) -> Result<Self> {
        let mut game = Cta {
            config,
            state: State::Game(Setup),
            deck_pool: vec![DeckInfo::new(Deck::default())],
            bottom_deck_index: 0,
            enforce_optimal_cut: false,
            cut_pair: false
        };

        game.apply_config()?;

        Ok(game)
    }

    fn apply_config(&mut self) -> Result<()> {

        // optimal cut is enforced when there are reverse wagers
        if self.config.get_base_config().get_wagers()
            .iter()
            .flat_map( |(_, wagers)| wagers.iter())
            .any( |wager| *wager.get_wager_type() == CtaWagerType::Reverse) {

                self.enforce_optimal_cut = true;
        }

        Ok(())
    }

    fn start_game(&mut self) -> Result<()> {
        if self.state != Game(Setup) {
            Err(InvalidTransition.into())
        } else {
            Ok(())
        }
    }

    fn end_game(&mut self) -> Result<()> {
        if self.state == Game(Setup) || self.state == Game(Ended) {
            Err(InvalidTransition.into())
        } else {
            Ok(())
        }
    }

    // card_index denotes the cut is made behind the card indexed by this value
    // ex. if a deck is [2D, 6C], a card_index of 0 means splitting the deck into [2D] and [6C]
    fn make_cut(&mut self, deck_index: usize, card_index: usize) -> Result<()> {

        if !self.can_cut_deck(deck_index) || !self.can_cut_at(deck_index, card_index) {
            return Err(InvalidCut(deck_index, card_index).into());
        }

        // perform the cut

        let new_deck = self.deck_pool[deck_index].deck.split(card_index);

        self.deck_pool.push(
            DeckInfo::new_with_info(new_deck, true, self.deck_pool[deck_index].bottom_reveal)
        );

        self.deck_pool[deck_index].bottom_reveal = true;

        if self.bottom_deck_index == deck_index {
            self.bottom_deck_index = self.deck_pool.len() - 1;
        }

        // check win condition
        let new_deck = &self.deck_pool[self.deck_pool.len() - 1].deck;
        let old_deck = &self.deck_pool[deck_index].deck;

        if old_deck.get_bottom().get_rank() == 1 || new_deck.get_top().get_rank() == 1 {
            if  old_deck.get_bottom().get_rank() == new_deck.get_top().get_rank() {
                self.cut_pair = true;
            }

            //win condition met, game ends
            self.state = Game(Ended);
        }

        // fix up the decks if optimal cut is enforced
        if self.enforce_optimal_cut {
            todo!()
        }

        Ok(())
    }

    fn can_cut_deck(&self, deck_index: usize) -> bool {

        // cannot cut a deck that does not exist
        if deck_index >= self.deck_pool.len() {
            return false;
        }

        if self.enforce_optimal_cut {

            // Cutting a deck with 2 cards or below is not optimal
            if self.deck_pool[deck_index].deck.total_len() <= 2 {
                return false;
            }
        }

        true
    }

    // an optimal cut is a cut which reveals two previously unseen cards
    fn can_cut_at(&self, deck_index: usize, card_index: usize) -> bool {

        let deck = &self.deck_pool[deck_index];
        if self.enforce_optimal_cut {

            if card_index >= deck.deck.total_len() - 1 {
                // Cannot cut after the last card
                return false;
            }

            if deck.top_reveal && card_index == 0 {
                return false;
            }

            if deck.bottom_reveal && card_index == deck.deck.total_len() - 2 {
                return false;
            }
        }

        true
    }

    fn fix_deck() {
        todo!()
    }
}

impl Serialize for Cta {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer {
        todo!()
    }
}

#[typetag::serialize]
impl Game for Cta {
    fn get_type(&self) -> GameType {
        GameType::Cta
    }

    fn transition(&mut self, transition: Transition) -> Result<()> {
        match &transition {
            Transition::Game(Start) => {
                self.start_game()?;
                self.state = Cta(AwaitCut);
            },
            Transition::Game(End) => {
                self.end_game()?;
                self.state = Game(Ended);
            },
            Transition::Cta(Cut {deck_index, position}) => self.make_cut(*deck_index, *position)?,
            _ => return Err(InvalidTransition.into())
        }

        Ok(())
    }

    fn get_valid_transitions(&self) -> Vec<Transition> {

        let mut transitions = Vec::new();

        match &self.state {
            Game(Setup) => transitions.push(Transition::Game(Start)),
            Game(Started) => transitions.push(Transition::Game(End)),
            Cta(AwaitCut) => {
                transitions.push(Transition::Game(End));
                transitions.push(Transition::Cta(Cut {deck_index: 0, position: 0}));
            },
            _ => {}
        }

        transitions
    }

    fn get_payout(&self) -> Result<Vec<Payout>> {
        let mut payouts: Vec<Payout> = Vec::new();

        if self.state != State::Game(Ended) {
            return Ok(payouts);
        }

        let mut house_payout = 0;

        for (player, wagers) in self.config.get_base_config().get_wagers() {
            for wager in wagers {
                let payout = match wager.get_wager_type() {
                    CtaWagerType::Forward => {
                        // Forward wagers are per cut. Payout amount * odds - wager * failed cuts

                        let win_payout: i32 = match self.cut_pair {
                            true => wager.get_amount() * self.config.get_odds().get_pair(),
                            false => wager.get_amount() * self.config.get_odds().get_forward()
                        };

                        // deck pool size - 1 is the number of failed cuts
                        win_payout - ( (self.deck_pool.len() - 1) as i32 * wager.get_amount() )
                    },

                    CtaWagerType::Reverse => {
                        // Reverse wagers lose odds * wager amount first, and then pay wager amount per failed cut

                        (self.deck_pool.len() - 1) as i32 * wager.get_amount() - wager.get_amount() * self.config.get_odds().get_reverse()
                    }
                };

                if payout != 0 {
                    payouts.push(Payout::new(player.get_id(), Some(wager), payout)?);
                    house_payout -= payout;
                }
            }
        }

        if house_payout != 0 {
            payouts.push(
                Payout::new::<FtsWagerType>(
                    self.config.get_base_config().get_house_id(), None, house_payout
                )?
            );
        }

        Ok(payouts)
    }
}