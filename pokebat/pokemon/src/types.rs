use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Type {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

impl Type {
    pub fn get_random_type() -> Type {
        let random_number: u32 = rand::thread_rng().gen_range(0..=17);
        Type::from(random_number)
    }
}

impl From<u32> for Type {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::Fire,
            2 => Self::Water,
            3 => Self::Electric,
            4 => Self::Grass,
            5 => Self::Ice,
            6 => Self::Fighting,
            7 => Self::Poison,
            8 => Self::Ground,
            9 => Self::Flying,
            10 => Self::Psychic,
            11 => Self::Bug,
            12 => Self::Rock,
            13 => Self::Ghost,
            14 => Self::Dragon,
            15 => Self::Dark,
            16 => Self::Steel,
            17 => Self::Fairy,
            _ => unreachable!(),
        }
    }
}
