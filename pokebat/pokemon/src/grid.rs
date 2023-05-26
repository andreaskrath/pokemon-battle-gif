use crate::creature::Pokemon;

pub struct BattleGrid {
    pokemons: Vec<Pokemon>,
    frames: Vec<Vec<u8>>,
    width: u32,
    height: u32,
}

impl BattleGrid {
    pub fn new(width: u32, height: u32) -> Self {
        let mut starting_grid: Vec<Pokemon> = Vec::new();
        for _ in 0..width * height {
            starting_grid.push(Pokemon::new());
        }
        Self {
            pokemons: starting_grid,
            frames: Vec::new(),
            width,
            height,
        }
    }

    pub fn simulate() {}
}
