use crate::types::Type;

#[derive(Debug)]
pub struct Pokemon {
    pub health: u32,
    pub typ: Type,
    pub damage: u32,
}

impl Pokemon {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            health: 20,
            typ: Type::get_random_type(),
            damage: 10,
        }
    }

    pub fn get_rgb(&self) -> (u8, u8, u8) {
        match self.typ {
            Type::Normal => (168, 167, 122),
            Type::Fire => (238, 129, 48),
            Type::Water => (99, 144, 240),
            Type::Electric => (247, 208, 44),
            Type::Grass => (122, 199, 76),
            Type::Ice => (150, 217, 214),
            Type::Fighting => (194, 46, 40),
            Type::Poison => (163, 62, 161),
            Type::Ground => (226, 191, 101),
            Type::Flying => (169, 143, 243),
            Type::Psychic => (249, 85, 135),
            Type::Bug => (166, 185, 26),
            Type::Rock => (182, 161, 54),
            Type::Ghost => (115, 87, 151),
            Type::Dragon => (111, 53, 252),
            Type::Dark => (112, 87, 70),
            Type::Steel => (183, 183, 206),
            Type::Fairy => (214, 133, 173),
        }
    }
}
