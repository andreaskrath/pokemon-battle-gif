use crate::types::{DamageMultiplier, Type};

#[derive(Debug)]
pub struct Pokemon {
    pub health: i32,
    pub typ: Type,
    pub damage: i32,
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

    pub fn attack(&self, other: &mut Pokemon, multiplier_type: &DamageMultiplier) {
        match *multiplier_type {
            DamageMultiplier::No => {} // No reason to do a calculation with no effect
            DamageMultiplier::Half => other.health -= self.damage / 2,
            DamageMultiplier::Normal => other.health -= self.damage,
            DamageMultiplier::Double => other.health -= self.damage * 2,
        }

        if other.health <= 0 {
            other.reset_health();
            other.typ = self.typ;
        }
    }

    #[inline(always)]
    pub fn reset_health(&mut self) {
        self.health = 20
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

    pub fn get_matchup_effect(&self, other: &Pokemon) -> DamageMultiplier {
        use Type::*;
        match self.typ {
            Normal => other.typ.normal_matchup(),
            Fire => other.typ.fire_matchup(),
            Water => other.typ.water_matchup(),
            Electric => other.typ.electric_matchup(),
            Grass => other.typ.grass_matchup(),
            Ice => other.typ.ice_matchup(),
            Fighting => other.typ.fighting_matchup(),
            Poison => other.typ.poison_matchup(),
            Ground => other.typ.ground_matchup(),
            Flying => other.typ.flying_matchup(),
            Psychic => other.typ.psychic_matchup(),
            Bug => other.typ.bug_matchup(),
            Rock => other.typ.rock_matchup(),
            Ghost => other.typ.ghost_matchup(),
            Dragon => other.typ.dragon_matchup(),
            Dark => other.typ.dark_matchup(),
            Steel => other.typ.steel_matchup(),
            Fairy => other.typ.fairy_matchup(),
        }
    }
}
