#[cfg_attr(test, derive(Debug, PartialEq))]
enum State {
    Mario,
    SuperMario,
    FireMario,
    CapeMario,
    StarMario,
}

enum Power {
    Feather,
    Fire,
    Mushroom,
    Star,
}

struct Mario {
    state: State,
}

impl Mario {
    fn new(state: State) -> Mario {
        Mario { state }
    }

    fn print_state(&self) {
        match self.state {
            State::Mario => println!("Mario"),
            State::SuperMario => println!("Super Mario"),
            State::FireMario => println!("Fire Mario"),
            State::CapeMario => println!("Cape Mario"),
            State::StarMario => println!("Star Mario"),
        }
    }

    fn collect_power(&mut self, power: Power) {
        match (&self.state, power) {
            (State::Mario, Power::Mushroom) => self.state = State::SuperMario,
            (_, Power::Fire) => self.state = State::FireMario,
            (_, Power::Feather) => self.state = State::CapeMario,
            (_, Power::Star) => self.state = State::StarMario,
            (_, Power::Mushroom) => {}
        }
    }

    fn take_damage(&mut self) {
        match self.state {
            State::Mario => {
                println!("Game Over");
            }
            State::SuperMario => self.state = State::Mario,
            State::FireMario => self.state = State::SuperMario,
            State::CapeMario => self.state = State::SuperMario,
            State::StarMario => self.state = State::StarMario,
        }
    }
}

pub fn run_game_states() {
    let mut player = Mario::new(State::Mario);

    player.print_state();

    player.collect_power(Power::Mushroom);
    player.print_state();

    player.collect_power(Power::Fire);
    player.print_state();

    player.take_damage();
    player.print_state();

    player.take_damage();
    player.print_state();

    player.take_damage();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect() {
        let mut player = Mario::new(State::Mario);

        player.collect_power(Power::Mushroom);
        assert_eq!(player.state, State::SuperMario);

        player.collect_power(Power::Fire);
        assert_eq!(player.state, State::FireMario);

        player.collect_power(Power::Feather);
        assert_eq!(player.state, State::CapeMario);

        player.collect_power(Power::Star);
        assert_eq!(player.state, State::StarMario);
    }

    #[test]
    fn test_take_damage_as_mario() {
        let mut player = Mario::new(State::Mario);

        player.take_damage();
        assert_eq!(player.state, State::Mario);
    }

    #[test]
    fn test_take_damage_as_super_mario() {
        let mut player = Mario::new(State::SuperMario);

        player.take_damage();
        assert_eq!(player.state, State::Mario);
    }

    #[test]
    fn test_take_damage_as_fire_mario() {
        let mut player = Mario::new(State::FireMario);

        player.take_damage();
        assert_eq!(player.state, State::SuperMario);
    }

    #[test]
    fn test_take_damage_as_cape_mario() {
        let mut player = Mario::new(State::CapeMario);

        player.take_damage();
        assert_eq!(player.state, State::SuperMario);
    }

    #[test]
    fn test_take_damage_as_star_mario() {
        let mut player = Mario::new(State::StarMario);

        player.take_damage();
        assert_eq!(player.state, State::StarMario);
    }
}
