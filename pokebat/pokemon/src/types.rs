use rand::Rng;

/// An array of the four types of damage multipliers, ordered to favor good matchups.
pub const DAMAGE_MULTIPLIER_ARRAY: &[DamageMultiplier] = &[
    DamageMultiplier::Double,
    DamageMultiplier::Normal,
    DamageMultiplier::Half,
    DamageMultiplier::No,
];

const NORMAL_HALF: &[Type] = &[Type::Rock, Type::Steel];
const NORMAL_NO: &[Type] = &[Type::Ghost];

const FIRE_DOUBLE: &[Type] = &[Type::Grass, Type::Ice, Type::Bug, Type::Steel];
const FIRE_HALF: &[Type] = &[Type::Fire, Type::Water, Type::Rock, Type::Dragon];

const WATER_DOUBLE: &[Type] = &[Type::Fire, Type::Ground, Type::Rock];
const WATER_HALF: &[Type] = &[Type::Water, Type::Grass, Type::Dragon];

const ELECTRIC_DOUBLE: &[Type] = &[Type::Water, Type::Flying];
const ELECTRIC_HALF: &[Type] = &[Type::Electric, Type::Grass, Type::Dragon];
const ELECTRIC_NO: &[Type] = &[Type::Ground];

const GRASS_DOUBLE: &[Type] = &[Type::Water, Type::Ground, Type::Rock];
const GRASS_HALF: &[Type] = &[
    Type::Fire,
    Type::Grass,
    Type::Poison,
    Type::Flying,
    Type::Bug,
    Type::Dragon,
    Type::Steel,
];

const ICE_DOUBLE: &[Type] = &[Type::Grass, Type::Ground, Type::Flying, Type::Dragon];
const ICE_HALF: &[Type] = &[Type::Fire, Type::Water, Type::Ice, Type::Steel];

const FIGHTING_DOUBLE: &[Type] = &[Type::Normal, Type::Ice, Type::Rock, Type::Dark, Type::Steel];
const FIGHTING_HALF: &[Type] = &[
    Type::Poison,
    Type::Flying,
    Type::Psychic,
    Type::Bug,
    Type::Fairy,
];
const FIGHTING_NO: &[Type] = &[Type::Ghost];

const POISON_DOUBLE: &[Type] = &[Type::Grass, Type::Fairy];
const POISON_HALF: &[Type] = &[Type::Poison, Type::Ground, Type::Rock, Type::Ghost];
const POISON_NO: &[Type] = &[Type::Steel];

const GROUND_DOUBLE: &[Type] = &[
    Type::Fire,
    Type::Electric,
    Type::Poison,
    Type::Rock,
    Type::Steel,
];
const GROUND_HALF: &[Type] = &[Type::Grass, Type::Bug];
const GROUND_NO: &[Type] = &[Type::Flying];

const FLYING_DOUBLE: &[Type] = &[Type::Grass, Type::Fighting, Type::Bug];
const FLYING_HALF: &[Type] = &[Type::Electric, Type::Rock, Type::Steel];

const PSYCHIC_DOUBLE: &[Type] = &[Type::Fighting, Type::Poison];
const PSYCHIC_HALF: &[Type] = &[Type::Psychic, Type::Steel];
const PSYCHIC_NO: &[Type] = &[Type::Dark];

const BUG_DOUBLE: &[Type] = &[Type::Grass, Type::Psychic, Type::Dark];
const BUG_HALF: &[Type] = &[
    Type::Fire,
    Type::Fighting,
    Type::Poison,
    Type::Flying,
    Type::Ghost,
    Type::Steel,
    Type::Fairy,
];

const ROCK_DOUBLE: &[Type] = &[Type::Fire, Type::Ice, Type::Flying, Type::Bug];
const ROCK_HALF: &[Type] = &[Type::Fighting, Type::Ground, Type::Steel];

const GHOST_DOUBLE: &[Type] = &[Type::Psychic, Type::Ghost];
const GHOST_HALF: &[Type] = &[Type::Dark];
const GHOST_NO: &[Type] = &[Type::Normal];

const DRAGON_DOUBLE: &[Type] = &[Type::Dragon];
const DRAGON_HALF: &[Type] = &[Type::Steel];
const DRAGON_NO: &[Type] = &[Type::Fairy];

const DARK_DOUBLE: &[Type] = &[Type::Psychic, Type::Ghost];
const DARK_HALF: &[Type] = &[Type::Fighting, Type::Dark, Type::Fairy];

const STEEL_DOUBLE: &[Type] = &[Type::Ice, Type::Rock, Type::Fairy];
const STEEL_HALF: &[Type] = &[Type::Fire, Type::Water, Type::Electric, Type::Steel];

const FAIRY_DOUBLE: &[Type] = &[Type::Fighting, Type::Dragon, Type::Dark];
const FAIRY_HALF: &[Type] = &[Type::Fire, Type::Poison, Type::Steel];

#[derive(Debug, PartialEq, Clone, Copy)]
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

    pub fn normal_matchup(&self) -> DamageMultiplier {
        if NORMAL_HALF.contains(self) {
            DamageMultiplier::Half
        } else if NORMAL_NO.contains(self) {
            DamageMultiplier::No
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn fire_matchup(&self) -> DamageMultiplier {
        if FIRE_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if FIRE_HALF.contains(self) {
            DamageMultiplier::Half
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn water_matchup(&self) -> DamageMultiplier {
        if WATER_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if WATER_HALF.contains(self) {
            DamageMultiplier::Half
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn electric_matchup(&self) -> DamageMultiplier {
        if ELECTRIC_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if ELECTRIC_HALF.contains(self) {
            DamageMultiplier::Half
        } else if ELECTRIC_NO.contains(self) {
            DamageMultiplier::No
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn grass_matchup(&self) -> DamageMultiplier {
        if GRASS_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if GRASS_HALF.contains(self) {
            DamageMultiplier::Half
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn ice_matchup(&self) -> DamageMultiplier {
        if ICE_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if ICE_HALF.contains(self) {
            DamageMultiplier::Half
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn fighting_matchup(&self) -> DamageMultiplier {
        if FIGHTING_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if FIGHTING_HALF.contains(self) {
            DamageMultiplier::Half
        } else if FIGHTING_NO.contains(self) {
            DamageMultiplier::No
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn poison_matchup(&self) -> DamageMultiplier {
        if POISON_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if POISON_HALF.contains(self) {
            DamageMultiplier::Half
        } else if POISON_NO.contains(self) {
            DamageMultiplier::No
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn ground_matchup(&self) -> DamageMultiplier {
        if GROUND_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if GROUND_HALF.contains(self) {
            DamageMultiplier::Half
        } else if GROUND_NO.contains(self) {
            DamageMultiplier::No
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn flying_matchup(&self) -> DamageMultiplier {
        if FLYING_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if FLYING_HALF.contains(self) {
            DamageMultiplier::Half
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn psychic_matchup(&self) -> DamageMultiplier {
        if PSYCHIC_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if PSYCHIC_HALF.contains(self) {
            DamageMultiplier::Half
        } else if PSYCHIC_NO.contains(self) {
            DamageMultiplier::No
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn bug_matchup(&self) -> DamageMultiplier {
        if BUG_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if BUG_HALF.contains(self) {
            DamageMultiplier::Half
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn rock_matchup(&self) -> DamageMultiplier {
        if ROCK_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if ROCK_HALF.contains(self) {
            DamageMultiplier::Half
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn ghost_matchup(&self) -> DamageMultiplier {
        if GHOST_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if GHOST_HALF.contains(self) {
            DamageMultiplier::Half
        } else if GHOST_NO.contains(self) {
            DamageMultiplier::No
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn dragon_matchup(&self) -> DamageMultiplier {
        if DRAGON_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if DRAGON_HALF.contains(self) {
            DamageMultiplier::Half
        } else if DRAGON_NO.contains(self) {
            DamageMultiplier::No
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn dark_matchup(&self) -> DamageMultiplier {
        if DARK_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if DARK_HALF.contains(self) {
            DamageMultiplier::Half
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn steel_matchup(&self) -> DamageMultiplier {
        if STEEL_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if STEEL_HALF.contains(self) {
            DamageMultiplier::Half
        } else {
            DamageMultiplier::Normal
        }
    }

    pub fn fairy_matchup(&self) -> DamageMultiplier {
        if FAIRY_DOUBLE.contains(self) {
            DamageMultiplier::Double
        } else if FAIRY_HALF.contains(self) {
            DamageMultiplier::Half
        } else {
            DamageMultiplier::Normal
        }
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

#[derive(PartialEq)]
pub enum DamageMultiplier {
    No,
    Half,
    Normal,
    Double,
}
