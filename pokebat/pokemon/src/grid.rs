use std::{collections::HashMap, fs::File};

use gif::{Encoder, Frame};

use crate::{args::Cli, pokemon::Pokemon};

const FRAME_DELAY: u16 = 25;

pub struct BattleGrid<'a> {
    pokemons: HashMap<(u16, u16), Pokemon>,
    frames: Vec<Frame<'a>>,
    frame_amount: u32,
    width: u16,
    height: u16,
}

impl<'a> From<Cli> for BattleGrid<'a> {
    fn from(args: Cli) -> Self {
        let mut starting_grid: HashMap<(u16, u16), Pokemon> = HashMap::new();
        for y_index in 0..args.height {
            for x_index in 0..args.width {
                let location = (x_index, y_index);
                let pokemon = Pokemon::new();
                if let Some(unknown_value) = starting_grid.insert(location, pokemon) {
                    unreachable!("there should be no collisions during the initial fill");
                }
            }
        }

        Self {
            pokemons: starting_grid,
            frames: Vec::new(),
            width: args.width,
            height: args.height,
            frame_amount: args.frame_amount,
        }
    }
}

impl<'a> BattleGrid<'a> {
    pub fn simulate(&mut self) {
        self.generate_frame();

        for _ in 0..self.frame_amount {
            //self.grid_battle();
            self.generate_frame();
        }

        self.create_gif();
    }

    fn grid_battle(&mut self) {
        for y_index in 0..self.height {
            for x_index in 0..self.width {
                let current_pokemon = self
                    .pokemons
                    .get(&(x_index, y_index))
                    .expect("failed to get current pokemon");

                let north_pokemon = self.pokemons.get_mut(&(x_index, y_index - 1));
                let south_pokemon = self.pokemons.get_mut(&(x_index, y_index + 1));
                let west_pokemon = self.pokemons.get_mut(&(x_index - 1, y_index));
                let east_pokemon = self.pokemons.get_mut(&(x_index + 1, y_index));
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

        let mut frame = gif::Frame::from_rgb(self.width, self.height, &pixels);
        frame.delay = FRAME_DELAY;
        self.frames.push(frame);
    }

    fn create_gif(&self) {
        let mut gif = File::create("../output/sample.gif").unwrap();
        let mut encoder = Encoder::new(&mut gif, self.width, self.height, &[]).unwrap();
        encoder.set_repeat(gif::Repeat::Infinite).unwrap();

        for frame in &self.frames {
            encoder.write_frame(frame).unwrap();
        }
    }
}
