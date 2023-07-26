use std::collections::VecDeque;
use rand::prelude::SliceRandom;

pub struct Deck {
    cards: VecDeque<Option<Movement>>,
    pub get_out_of_jail_free_owned: bool,
}

impl Deck {
    fn new(mut card_arr: [Option<Movement>; 16]) -> Deck {
        let mut rng = rand::thread_rng();
        card_arr.shuffle(&mut rng);

        Deck {
            cards: VecDeque::from(card_arr),
            get_out_of_jail_free_owned: false,
        }
    }

    pub fn new_chance_deck() -> Deck {
        let card_arr = [
            Some(Movement::Absolute(39)),
            Some(Movement::Absolute(0)),
            Some(Movement::Absolute(24)),
            Some(Movement::Absolute(11)),
            Some(Movement::Nearest(SpaceType::Railroad)),
            Some(Movement::Nearest(SpaceType::Railroad)),
            Some(Movement::Nearest(SpaceType::Utility)),
            None,
            Some(Movement::JailFree),
            Some(Movement::Relative(-3)),
            Some(Movement::Absolute(30)),
            None,
            None,
            Some(Movement::Absolute(5)),
            None,
            None,
        ];

        Deck::new(card_arr)
    }

    pub fn new_community_chest_deck() -> Deck {
        let card_arr = [
            Some(Movement::Absolute(0)),
            None,
            None,
            None,
            Some(Movement::JailFree),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ];

        Deck::new(card_arr)
    }

    pub fn draw(&mut self) -> Option<&Movement> {
        let card = self.cards.pop_front().unwrap();

        let index = match self.get_out_of_jail_free_owned {
            true => 14,
            false => 15,
        };
        
        if let Some(movement) = card.as_ref() {
            self.get_out_of_jail_free_owned = *movement == Movement::JailFree;
        }

        self.cards.insert(index, card);
        self.cards.get(index).unwrap().as_ref()
    }
}


#[derive(PartialEq)]
pub enum Movement {
    Absolute(i8),
    Relative(i8),
    Nearest(SpaceType),
    JailFree,
}

#[derive(PartialEq)]
pub enum SpaceType {
    Railroad,
    Utility,
}