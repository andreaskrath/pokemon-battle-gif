use std::fs::File;

use gif::{Encoder, Frame};

use crate::pokemon::Pokemon;

const FRAME_DELAY: u16 = 25;

pub struct BattleGrid<'a> {
    pokemons: Vec<Pokemon>,
    frames: Vec<Frame<'a>>,
    frame_amount: u32,
    width: u16,
    height: u16,
}

impl<'a> BattleGrid<'a> {
    pub fn new(width: u16, height: u16, frame_amount: u32) -> Self {
        let mut starting_grid: Vec<Pokemon> = Vec::new();
        for _ in 0..(width as u64 * height as u64) {
            starting_grid.push(Pokemon::new());
        }
        Self {
            pokemons: starting_grid,
            frames: Vec::new(),
            width,
            height,
            frame_amount,
        }
    }

    pub fn simulate(&mut self) {
        for _ in 0..self.frame_amount {
            self.generate_frame();
        }

        self.create_gif();
    }

    fn generate_frame(&mut self) {
        let mut pixels: Vec<u8> = Vec::new();
        for pokemon in &mut self.pokemons {
            let (red, green, blue) = pokemon.get_rgb();
            pixels.append(&mut vec![red, green, blue]);
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
