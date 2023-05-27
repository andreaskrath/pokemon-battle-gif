use std::{collections::HashMap, fs::File};

use gif::{Encoder, Frame};
use rand::{thread_rng, Rng};

use crate::{args::Cli, pokemon::Pokemon, types::DAMAGE_MULTIPLIER_ARRAY};

const FRAME_DELAY: u16 = 25;

pub struct BattleGrid<'a> {
    pokemons: HashMap<(i32, i32), Pokemon>,
    frames: Vec<Frame<'a>>,
    frame_amount: u32,
    width: i32,
    height: i32,
}

impl<'a> From<Cli> for BattleGrid<'a> {
    fn from(args: Cli) -> Self {
        let mut starting_grid: HashMap<(i32, i32), Pokemon> = HashMap::new();
        for y_index in 0..args.height {
            for x_index in 0..args.width {
                let location = (x_index as i32, y_index as i32);
                let pokemon = Pokemon::new();
                if starting_grid.insert(location, pokemon).is_some() {
                    unreachable!("there should be no collisions during the initial fill");
                }
            }
        }

        Self {
            pokemons: starting_grid,
            frames: Vec::new(),
            width: args.width as i32,
            height: args.height as i32,
            frame_amount: args.frame_amount,
        }
    }
}

impl<'a> BattleGrid<'a> {
    pub fn simulate(&mut self) {
        self.generate_frame();

        for _ in 0..self.frame_amount {
            self.grid_battle();
            self.generate_frame();
        }

        self.create_gif();
    }

    fn grid_battle(&mut self) {
        for y_index in 0..self.height {
            for x_index in 0..self.width {
                let mut pokemons: Vec<((i32, i32), Pokemon)> = Vec::new();
                let attacking_position = (x_index, y_index);
                let north_position = (x_index, y_index - 1);
                let south_position = (x_index, y_index + 1);
                let west_position = (x_index - 1, y_index);
                let east_position = (x_index + 1, y_index);

                let Some(attacking_pokemon) = self.pokemons.remove(&(attacking_position)) else {
                    panic!("could not get current pokemon");
                };

                if let Some(north_pokemon) = self.pokemons.remove(&north_position) {
                    pokemons.push((north_position, north_pokemon));
                }
                if let Some(south_pokemon) = self.pokemons.remove(&south_position) {
                    pokemons.push((south_position, south_pokemon));
                }
                if let Some(west_pokemon) = self.pokemons.remove(&west_position) {
                    pokemons.push((west_position, west_pokemon));
                }
                if let Some(east_pokemon) = self.pokemons.remove(&east_position) {
                    pokemons.push((east_position, east_pokemon));
                }

                for _ in 0..5 {
                    let index_a = thread_rng().gen_range(0..pokemons.len());
                    let index_b = thread_rng().gen_range(0..pokemons.len());
                    pokemons.swap(index_a, index_b);
                }

                'outer: for multiplier_type in DAMAGE_MULTIPLIER_ARRAY {
                    for (_, current_pokemon) in pokemons.iter_mut() {
                        if attacking_pokemon.get_matchup_effect(current_pokemon) == *multiplier_type
                        {
                            attacking_pokemon.attack(current_pokemon, multiplier_type);
                            break 'outer;
                        }
                    }
                }

                self.pokemons.insert(attacking_position, attacking_pokemon);
                for (location, pokemon) in pokemons.into_iter() {
                    self.pokemons.insert(location, pokemon);
                }
            }
        }
    }

    /// Generates a gif frame based on the current contents of the `pokemons` field vector.
    fn generate_frame(&mut self) {
        let mut pixels: Vec<u8> = Vec::new();

        for y_index in 0..self.height {
            for x_index in 0..self.width {
                match self.pokemons.get(&(x_index, y_index)) {
                    Some(pokemon) => {
                        let (red, green, blue) = pokemon.get_rgb();
                        pixels.append(&mut vec![red, green, blue]);
                    }
                    None => unreachable!(
                        "there should not be any None values when the grid is bound checked"
                    ),
                }
            }
        }

        let mut frame = gif::Frame::from_rgb(self.width as u16, self.height as u16, &pixels);
        frame.delay = FRAME_DELAY;
        self.frames.push(frame);
    }

    fn create_gif(&self) {
        let mut gif = File::create("sample.gif").unwrap();
        let mut encoder =
            Encoder::new(&mut gif, self.width as u16, self.height as u16, &[]).unwrap();
        encoder.set_repeat(gif::Repeat::Infinite).unwrap();

        for frame in &self.frames {
            encoder.write_frame(frame).unwrap();
        }
    }
}
