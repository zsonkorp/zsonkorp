use crate::deck::Deck;
use crate::config::{CtaWagerType, FtsWagerType};
use crate::config;
use crate::game::{Game, GameType};
use anyhow::Result;
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

struct DeckHelper {
    deck: Deck,
    from_index: usize
}

impl DeckHelper {
    pub fn new(deck: Deck, from_index: usize) -> Self {
        DeckHelper { deck, from_index }
    }
}

pub struct Cta {
    config: config::Cta,
    state: State,
    deck_pool: Vec<DeckHelper>,
    bottom_deck_index: usize,
    enforce_optimal_cut: bool,
    cut_pair: bool,
    orig_top_revealed: bool,
    orig_bottom_revealed: bool
}

impl Cta {
    pub fn new(config: config::Cta) -> Result<Self> {
        let mut game = Cta {
            config,
            state: State::Game(Setup),
            deck_pool: vec![DeckHelper::new(Deck::default(), 0)],
            bottom_deck_index: 0,
            enforce_optimal_cut: false,
            cut_pair: false,
            orig_bottom_revealed: false,
            orig_top_revealed: false
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

    fn make_cut(&mut self, deck_index: &usize, position: &usize) -> Result<()> {

        if *deck_index >= self.deck_pool.len() ||
            // cutting above the top card or below the bottom card is always invalid as it does not create an additional deck, thus not a cut
            *position == 0 || *position >= self.deck_pool[*deck_index].deck.total_len() {
            return Err(InvalidCut(*deck_index, *position).into());
        }

        // an optimal cut is a cut which reveals two more previously unseen cards
        // cutting the second card is optimal only if the top card has never been revealed ie. top card of the original deck
        // cutting the bottom card is optimal only if the bottom card has never been revealed ie. bottom card of the original deck
        if self.enforce_optimal_cut {
            if *position == 1 && (*deck_index != 0 || self.orig_top_revealed) {
                return Err(UnoptimalCut(*position).into());
            } else if *position == self.deck_pool[*deck_index].deck.total_len() - 1 && (*deck_index == self.bottom_deck_index || self.orig_bottom_revealed) {
                return Err(UnoptimalCut(*position).into());
            }
        }

        // perform cut and update bookkeeping structures
        let old_deck_len = self.deck_pool[*deck_index].deck.total_len();
        let new_deck = self.deck_pool[*deck_index].deck.split(*position);
        self.deck_pool.push(
            DeckHelper::new(
                new_deck,
                *deck_index
            )
        );

        if self.bottom_deck_index == *deck_index {
            self.bottom_deck_index = self.deck_pool.len() - 1;
        }

        if *position == 1 {
            self.orig_top_revealed = true;
        }

        if *position == old_deck_len - 1 {
            self.orig_bottom_revealed = true;
        }

        // check win condition
        let new_deck = &self.deck_pool[self.deck_pool.len() - 1].deck;
        let old_deck = &self.deck_pool[*deck_index].deck;

        if old_deck.get_bottom().get_rank() == 1 || new_deck.get_top().get_rank() == 1 {
            if  old_deck.get_bottom().get_rank() == new_deck.get_top().get_rank() {
                self.cut_pair = true;
            }

            //win condition met, game ends
            self.state = Game(Ended);
        }

        Ok(())
    }
}

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
            Transition::Cta(Cut {deck_index, position}) => self.make_cut(deck_index, position)?,
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