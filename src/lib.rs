use rand::Rng;


mod deck;
use deck::*;


struct Dice(i8, i8);

impl Dice {
    fn new() -> Dice {
        let mut dice = Dice(1, 1);
        dice.roll();
        dice
    }

    fn roll(&mut self) {
        self.0 = rand::thread_rng().gen_range(1..=6);
        self.1 = rand::thread_rng().gen_range(1..=6);
    }

    fn value(&self) -> i8 {
        self.0 + self.1
    }

    fn is_doubles(&self) -> bool {
        self.0 == self.1
    }
}


#[derive(Debug, PartialEq)]
struct Space(i8);

impl Space {
    fn new() -> Space {
        Space(0)
    }

    fn new_at(x: i8) -> Space {
        let mut space = Space::new();
        space.move_to(x);
        space
    }

    fn move_by(&mut self, x: i8) {
        if x > 12 {
            panic!("forward movement shoudln't exceed max dice roll");
        }
        self.0 += x;
        if self.0 > 39 {
            self.0 -= 40;
        } else if self.0 < 0 {
            self.0 += 40;
        }
    }

    fn move_to(&mut self, x: i8) {
        if x < 0 || x > 39 {
            panic!("attempted to move out of bounds")
        }

        self.0 = x;
    }

    fn value(&self) -> i8 {
        self.0
    }
}


pub struct Game {
    dice: Dice,
    chance_deck: Deck,
    community_chest_deck: Deck,
    position: Space,
    escape_attempts: u8,
}

impl Game {
    pub fn new() -> Game {
        let dice = Dice::new();
        let chance_deck = Deck::new_chance_deck();
        let community_chest_deck = Deck::new_community_chest_deck();
        let position = Space::new();

        Game {
            dice,
            chance_deck,
            community_chest_deck,
            position,
            escape_attempts: 0,
        }
    }

    pub fn take_turn(&mut self) -> Vec<i8> {
        let mut positions = Vec::new();

        if self.position.value() == 30 {
            self.dice.roll();

            if self.dice.is_doubles() {
                self.position.move_to(10);
                self.position.move_by(self.dice.value());
                positions.push(self.position.value());

                self.escape_attempts = 0;

                if let Some(p) = self.evaluate_space_action() {
                    positions.push(p)
                }
            } else {
                self.escape_attempts += 1;

                if self.escape_attempts == 3 {
                    self.position.move_to(10);
                    positions.push(self.position.value());

                    self.escape_attempts = 0;
                }
            }
        } else {
            for _ in 0..2 {
                self.dice.roll();
                self.position.move_by(self.dice.value());
                positions.push(self.position.value());

                if let Some(p) = self.evaluate_space_action() {
                    positions.push(p)
                }

                if !self.dice.is_doubles() {
                    return positions
                }
            }

            self.dice.roll();
            if self.dice.is_doubles() {
                self.position.move_to(30);
                positions.push(self.position.value());
            } else {
                self.position.move_by(self.dice.value());
                positions.push(self.position.value());

                if let Some(p) = self.evaluate_space_action() {
                    positions.push(p)
                }
            }
        }

        positions
    }

    fn evaluate_space_action(&mut self) -> Option<i8> {
        let deck_for_space = match self.position.value() {
            2 | 17 | 33 => Some(&mut self.community_chest_deck),
            7 | 22 | 36 => Some(&mut self.chance_deck),
            _ => None,
        };
 //       let deck = deck_for_space?;
        let movement = deck_for_space?.draw()?;
        match movement {
            Movement::Absolute(x) => self.position.move_to(*x),
            Movement::Relative(x) => self.position.move_by(*x),
            Movement::Nearest(space_type) => {
                let x = match space_type {
                    SpaceType::Railroad => {
                        match self.position.value() {
                            7 => 15,
                            22 => 25,
                            36 => 5,
                            _ => panic!("impossible"),
                        }
                    },
                    SpaceType::Utility => {
                        match self.position.value() {
                            7 => 12,
                            22 => 28,
                            36 => 12,
                            _ => panic!("impossible"),
                        }
                    }
                };
                self.position.move_to(x);
            },
            Movement::JailFree => {
                self.community_chest_deck.get_out_of_jail_free_owned = true;
                return None
            }
        };

        Some(self.position.value())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_valid_spaces() {
        for i in 0..40 {
            let _space = Space::new_at(i);
        }
    }

    #[test]
    #[should_panic]
    fn new_invalid_space() {
        let _space = Space::new_at(40);
    }

    #[test]
    fn forward_move_by() {
        let mut space = Space::new();
        space.move_by(5);
        assert_eq!(space, Space(5));
    }

    #[test]
    fn backward_move_by() {
        let mut space = Space::new_at(10);
        space.move_by(-5);
        assert_eq!(space, Space(5));
    }

    #[test]
    fn forward_move_by_past_go() {
        let mut space = Space::new_at(35);
        space.move_by(10);
        assert_eq!(space, Space(5))
    }

    #[test]
    fn backward_move_by_past_go() {
        let mut space = Space::new_at(2);
        space.move_by(-3);
        assert_eq!(space, Space(39));
    }

    #[test]
    fn all_valid_move_to() {
        let mut space = Space::new();
        for i in 0..40 {
            space.move_to(i);
            assert_eq!(space, Space(i));
        }
    }

    #[test]
    #[should_panic]
    fn move_to_out_of_bounds_positive() {
        let mut space = Space::new();
        space.move_to(40);
    }

    #[test]
    #[should_panic]
    fn move_to_out_of_bounds_negative() {
        let mut space = Space::new();
        space.move_to(-1);
    }

    #[test]
    #[should_panic]
    fn positive_move_by_too_big() {
        let mut space = Space::new();
        space.move_by(13);
    }
}