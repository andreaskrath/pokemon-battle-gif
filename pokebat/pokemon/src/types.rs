use rand::Rng;

const NORMAL_HALF: &[Type] = &[Type::Rock, Type::Steel];
const NORMAL_NO: &[Type] = &[Type::Ghost];

const FIRE_DOUBLE: &[Type] = &[Type::Grass, Type::Ice, Type::Bug, Type::Steel];
const FIRE_HALF: &[Type] = &[Type::Fire, Type::Water, Type::Rock, Type::Dragon];

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

    pub fn get_matchup_effect(&self, other: Type) -> TypeDamageMultiplier {
        use Type::*;
        match *self {
            Normal => other.normal_matchup(),
            Fire => other.fire_matchup(),
            _ => unreachable!(),
        }
    }

    fn normal_matchup(&self) -> TypeDamageMultiplier {
        if NORMAL_HALF.contains(self) {
            TypeDamageMultiplier::HalfDamage
        } else if NORMAL_NO.contains(self) {
            TypeDamageMultiplier::NoDamage
        } else {
            TypeDamageMultiplier::NormalDamage
        }
    }

    fn fire_matchup(&self) -> TypeDamageMultiplier {
        if FIRE_DOUBLE.contains(self) {
            TypeDamageMultiplier::DoubleDamage
        } else if FIRE_HALF.contains(self) {
            TypeDamageMultiplier::HalfDamage
        } else {
            TypeDamageMultiplier::NormalDamage
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

pub enum TypeDamageMultiplier {
    NoDamage,
    HalfDamage,
    NormalDamage,
    DoubleDamage,
}
