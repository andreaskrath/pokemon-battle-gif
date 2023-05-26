use crate::types::Type;

pub struct Pokemon {
    health: u32,
    typ: Type,
    damage: u32,
}

impl Pokemon {
    pub fn new() -> Self {
        Self {
            health: 20,
            typ: Type::get_random_type(),
            damage: 10,
        }
    }
}
